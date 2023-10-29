// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

package app.tauri.nfc

import android.app.Activity
import android.app.PendingIntent
import android.content.Intent
import android.content.IntentFilter
import android.nfc.NdefMessage
import android.nfc.NdefRecord
import android.nfc.NfcAdapter
import android.nfc.Tag
import android.nfc.tech.Ndef
import android.nfc.tech.NdefFormatable
import android.os.Build
import android.os.Parcelable
import android.webkit.WebView
import app.tauri.Logger
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSArray
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import org.json.JSONArray
import java.io.IOException
import kotlin.concurrent.thread

sealed class NfcAction {
    object Read : NfcAction()
    data class Write(val message: NdefMessage) : NfcAction()
}

@InvokeArg
class ScanOptions {
    var keepSessionAlive: Boolean = false
}

@InvokeArg
class NDEFRecordData {
    var format: Short = 0
    var kind: ByteArray = ByteArray(0)
    var id: ByteArray = ByteArray(0)
    var payload: ByteArray = ByteArray(0)
}

@InvokeArg
class WriteOptions {
    var keepSessionAlive: Boolean = false
    lateinit var records: Array<NDEFRecordData>
}

class Session(
    val action: NfcAction,
    val invoke: Invoke,
    val keepAlive: Boolean,
    var tag: Tag? = null
)

@TauriPlugin
class NfcPlugin(private val activity: Activity) : Plugin(activity) {
    private lateinit var webView: WebView

    private var nfcAdapter: NfcAdapter? = null
    private var session: Session? = null

    override fun load(webView: WebView) {
        super.load(webView)
        this.webView = webView
        this.nfcAdapter = NfcAdapter.getDefaultAdapter(activity.applicationContext)
    }

    override fun onNewIntent(intent: Intent) {
        Logger.info("NFC", "onNewIntent")
        super.onNewIntent(intent)

        val extraTag = intent.getParcelableExtra<Tag>(NfcAdapter.EXTRA_TAG)

        extraTag?.let { tag ->
            session?.let {
                if (it.keepAlive) {
                    it.tag = tag
                }
            }

            when (session?.action) {
                is NfcAction.Read -> readTag(tag, intent)
                is NfcAction.Write -> thread {
                    if (session?.action is NfcAction.Write) {
                        try {
                            writeTag(tag, (session?.action as NfcAction.Write).message)
                            session?.invoke?.resolve()
                        } catch (e: Exception) {
                            session?.invoke?.reject(e.toString())
                        } finally {
                            if (this.session?.keepAlive != true) {
                                this.session = null
                                disableNFCInForeground()
                            }
                        }
                    }
                }

                else -> {}
            }
        }

    }

    override fun onPause() {
        disableNFCInForeground()
        super.onPause()
        Logger.info("NFC", "onPause")
    }

    override fun onResume() {
        super.onResume()
        Logger.info("NFC", "onResume")
        if (this.session != null) {
            enableNFCInForeground()
        }
    }

    private fun isAvailable(): Pair<Boolean, String?> {
        val available: Boolean
        var errorReason: String? = null

        if (this.nfcAdapter === null) {
            available = false
            errorReason = "Device does not have NFC capabilities"
        } else if (this.nfcAdapter?.isEnabled == false) {
            available = false
            errorReason = "NFC is disabled in device settings"
        } else {
            available = true
        }

        return Pair(available, errorReason)
    }

    @Command
    fun isAvailable(invoke: Invoke) {
        val ret = JSObject()
        ret.put("available", isAvailable().first)
        invoke.resolve(ret)
    }

    @Command
    fun scan(invoke: Invoke) {
        val status = isAvailable()
        if (!status.first) {
            invoke.reject("NFC unavailable: " + status.second)
            return
        }

        val args = invoke.parseArgs(ScanOptions::class.java)

        enableNFCInForeground()

        session = Session(NfcAction.Read, invoke, args.keepSessionAlive)
    }

    @Command
    fun write(invoke: Invoke) {
        val status = isAvailable()
        if (!status.first) {
            invoke.reject("NFC unavailable: " + status.second)
            return
        }

        val args = invoke.parseArgs(WriteOptions::class.java)

        val ndefRecords: MutableList<NdefRecord> = ArrayList()
        for (record in args.records) {
            ndefRecords.add(NdefRecord(record.format, record.kind, record.id, record.payload))
        }

        val message = NdefMessage(ndefRecords.toTypedArray())

        session?.let { session ->
            session.tag?.let {
                try {
                    writeTag(it, message)
                    invoke.resolve()
                } catch (e: Exception) {
                    invoke.reject(e.toString())
                } finally {
                    if (this.session?.keepAlive != true) {
                        this.session = null
                        disableNFCInForeground()
                    }
                }
            } ?: run {
                invoke.reject("connected tag not found, please wait for it to be available and then call write()")
            }
        } ?: run {
            enableNFCInForeground()
            session = Session(NfcAction.Write(message), invoke, args.keepSessionAlive)
            Logger.warn("NFC", "Write Mode Enabled")
        }
    }

    private fun readTag(tag: Tag, intent: Intent) {
        try {
            val rawMessages = intent.getParcelableArrayExtra(NfcAdapter.EXTRA_NDEF_MESSAGES)

            when (intent.action) {
                NfcAdapter.ACTION_NDEF_DISCOVERED -> {
                    // For some reason this one never triggers.
                    Logger.info("NFC", "new NDEF intent")
                    readTagInner(tag, rawMessages)
                }
                NfcAdapter.ACTION_TECH_DISCOVERED -> {
                    // For some reason this always triggers instead of NDEF_DISCOVERED even though we set ndef filters right now
                    Logger.info("NFC", "new TECH intent")
                    // TODO: handle different techs. Don't assume ndef.
                    readTagInner(tag, rawMessages)
                }
                NfcAdapter.ACTION_TAG_DISCOVERED -> {
                    // This should never trigger when an app handles NDEF and TECH
                    // TODO: Don't assume ndef.
                    readTagInner(tag, rawMessages)
                }
            }
        } catch (e: Exception) {
            session?.invoke?.reject("failed to read tag", e)
        } finally {
            if (this.session?.keepAlive != true) {
                this.session = null
            }
            // TODO this crashes? disableNFCInForeground()
        }
    }

    private fun readTagInner(tag: Tag?, rawMessages: Array<Parcelable>?) {
        // iOS part only reads the first message (? - there are 2 conflicting impls so not sure) so we do too. i think that covers most if not all use cases anyway.
        val ndefMessage = rawMessages?.get(0) as NdefMessage?

        val records = ndefMessage?.records ?: arrayOf()

        val jsonRecords = Array(records.size) { i -> recordToJson(records[i]) }

        val ret = JSObject()
        if (tag !== null) {
            ret.put("id", fromU8Array(tag.id))
            // There's also ndef.type which returns the ndef spec type which may be interesting to know too?
            ret.put("kind", JSArray.from(tag.techList))
        }
        ret.put("records", JSArray.from(jsonRecords))

        session?.invoke?.resolve(ret)
    }

    private fun writeTag(tag: Tag, message: NdefMessage) {
        // This should return tags that are already in ndef format
        val ndefTag = Ndef.get(tag)
        if (ndefTag !== null) {
            // We have to connect first to check maxSize.
            try {
                ndefTag.connect()
            } catch (e: IOException) {
                throw Exception("Couldn't connect to NFC tag", e)
            }

            if (ndefTag.maxSize < message.toByteArray().size) {
                throw Exception("The message is too large for the provided NFC tag")
            } else if (!ndefTag.isWritable) {
                throw Exception("NFC tag is read-only")
            } else {
                try {
                    ndefTag.writeNdefMessage(message)
                } catch (e: Exception) {
                    throw Exception("Couldn't write message to NFC tag", e)
                }
            }

            try {
                ndefTag.close()
            } catch (e: IOException) {
                Logger.error("failed to close tag", e)
            }

            return
        }

        // This should cover tags that are not yet in ndef format but can be converted
        val ndefFormatableTag = NdefFormatable.get(tag)
        if (ndefFormatableTag !== null) {
            try {
                ndefFormatableTag.connect()
                ndefFormatableTag.format(message)
            } catch (e: Exception) {
                throw Exception("Couldn't format tag as Ndef", e)
            }

            try {
                ndefFormatableTag.close()
            } catch (e: IOException) {
                Logger.error("failed to close tag", e)
            }

            return
        }


        // if we get to this line, the tag was neither Ndef nor NdefFormatable compatible
        throw Exception("Tag doesn't support Ndef format")
    }

    // TODO: Use ReaderMode instead of ForegroundDispatch
    private fun enableNFCInForeground() {
        val flag =
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_MUTABLE else PendingIntent.FLAG_UPDATE_CURRENT
        val pendingIntent = PendingIntent.getActivity(
            activity, 0,
            Intent(
                activity,
                activity.javaClass
            ).addFlags(Intent.FLAG_ACTIVITY_SINGLE_TOP),
            flag
        )

        // For some reason this is ignored
        val nfcIntentFilter = IntentFilter(NfcAdapter.ACTION_NDEF_DISCOVERED)
        val filters = arrayOf(nfcIntentFilter)

        // Probably also ignored, if not it should include more than just these 2 once we don't just read simple ndefs.
        val techLists =
            arrayOf(arrayOf(Ndef::class.java.name), arrayOf(NdefFormatable::class.java.name))

        nfcAdapter?.enableForegroundDispatch(activity, pendingIntent, filters, techLists)
    }

    private fun disableNFCInForeground() {
        activity.runOnUiThread {
            nfcAdapter?.disableForegroundDispatch(activity)
        }
    }
}

private fun fromU8Array(byteArray: ByteArray): JSONArray {
    val json = JSONArray()
    for (byte in byteArray) {
        json.put(byte)
    }
    return json
}

private fun recordToJson(record: NdefRecord): JSObject {
    val json = JSObject()
    json.put("tnf", record.tnf)
    json.put("kind", fromU8Array(record.type))
    json.put("id", fromU8Array(record.id))
    json.put("payload", fromU8Array(record.payload))
    return json
}