(function () {
  'use strict';

  // Shared IPC helpers
  var T = window.__TAURI__;
  if (!T) {
    console.error('Tauri API not available');
    return;
  }

  window.QRCTool = {
    invoke: function (cmd, args) {
      return T.core.invoke(cmd, args);
    },

    // Notify Rust that a window is closing
    notifyClose: function () {
      return T.core.invoke('notify_closed');
    },

    // Close current window
    closeWindow: function () {
      var w = window.__TAURI__.window.getCurrentWindow
        ? window.__TAURI__.window.getCurrentWindow()
        : null;
      if (w && w.close) {
        w.close();
      } else {
        window.close();
      }
    },

    // Get URL parameter
    getParam: function (name) {
      var params = new URLSearchParams(window.location.search);
      return params.get(name) || '';
    },

    // Copy text to clipboard
    copyText: function (text) {
      return navigator.clipboard.writeText(text).catch(function () {
        var ta = document.createElement('textarea');
        ta.value = text;
        ta.style.position = 'fixed';
        ta.style.left = '-9999px';
        document.body.appendChild(ta);
        ta.select();
        document.execCommand('copy');
        document.body.removeChild(ta);
      });
    },

    // Escape HTML
    escapeHTML: function (str) {
      var div = document.createElement('div');
      div.appendChild(document.createTextNode(str));
      return div.innerHTML;
    },

    // Check if text looks like a URL
    isURL: function (text) {
      return /^https?:\/\/.+/i.test(text);
    }
  };
})();
