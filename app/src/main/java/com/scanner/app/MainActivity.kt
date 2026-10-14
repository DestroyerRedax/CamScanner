package com.scanner.app

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import androidx.camera.lifecycle.ProcessCameraProvider
import androidx.core.content.ContextCompat

class MainActivity : AppCompatActivity() {

    init {
        System.loadLibrary("scanner_core")
    }

    external fun apply_scanner_filter(inputBytes: ByteArray, inputLen: Int, filterType: Int, outputLen: IntArray): ByteArray?
    external fun perform_ocr_on_image(inputBytes: ByteArray, inputLen: Int): String?
    external fun apply_watermark_to_image(inputBytes: ByteArray, inputLen: Int, watermarkText: String, outputLen: IntArray): ByteArray?
    external fun export_scans_to_pdf(outputPath: String): Boolean

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(android.R.layout.activity_list_item)

        val cameraProviderFuture = ProcessCameraProvider.getInstance(this)
        cameraProviderFuture.addListener({
            val cameraProvider = cameraProviderFuture.get()
            // Camera setup and preview initialization
        }, ContextCompat.getMainExecutor(this))
    }
}