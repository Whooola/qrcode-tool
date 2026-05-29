# QRCode Tool

系统全局二维码工具。常驻后台运行，在任意应用中双击 Ctrl 即可将选中文本转为二维码，或打开摄像头扫描二维码。

支持 Windows 7 SP1+ 和 macOS，安装包 < 10MB。

## 安装

从 Releases 页面下载对应平台安装包。

## 使用

**生成二维码**：任意应用中选中文字，双击 Ctrl → 弹出二维码窗口。

**扫描二维码**：确保无选中文字，双击 Ctrl → 摄像头窗口打开 → 对准绿色框线 → 识别成功展示结果。

**关闭窗口**：ESC / 双击Ctrl / 点击 × 按钮 / 点击窗口外部。

## 自定义热键

编辑 `~/.qrcode-tool/config.json`（首次手动创建）：

```json
{
  "hotkeys": [
    {
      "id": "my-trigger",
      "key": "Control",
      "press_count": 2,
      "timeout_ms": 500,
      "action": "smart-trigger"
    }
  ]
}
```

可选 action: `smart-trigger`（智能判断）、`generate-qr`（仅生成）、`scan-qr`（仅扫码）。
可选 key: `Control`、`Alt`、`Shift`、`Meta`。

## 平台说明

|              | Windows 10+            | Windows 7 SP1              | macOS                |
| ------------ | ---------------------- | -------------------------- | -------------------- |
| WebView      | Edge WebView2（自带）   | WebView2 Runtime（需安装）  | WKWebView（自带）     |
| 体积         | ~5 MB                  | ~7 MB（含 bootstrapper）   | ~6 MB                |

## 开发

```bash
npm install
cargo tauri dev     # 开发模式
cargo tauri build   # 生产构建
```

项目结构：`src-tauri/src/`（Rust 后端）、`src/`（前端 HTML/JS）、`src/lib/`（QR 库）。
