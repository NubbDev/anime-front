package me.nubbless.dev

import android.os.Bundle
import android.os.PersistableBundle
import androidx.core.view.WindowCompat

class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?, persistentState: PersistableBundle?) {
    super.onCreate(savedInstanceState, persistentState)
    println("Updating to remove system window insets.")
    WindowCompat.setDecorFitsSystemWindows(window, false)
  }
}