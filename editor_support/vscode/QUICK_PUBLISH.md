# Nova Language Extension v1.1.0 - Update Publish Guide (TW4 Organization)

## ✅ Extension Update Package Hazır!
- Dosya: `nova-language-extension-1.1.0.vsix` (329KB)
- Publisher: `tw4`
- Organization: TW4
- **Version**: 1.1.0 (Major Update)
- **Extension Name**: Nova Language Extension

## 🚀 PUBLISH ADIM ADIM:

### 1. Publisher Account Oluştur (5 dakika)
```bash
# Web browser'da şu adrese git:
open https://marketplace.visualstudio.com/manage
```
- Microsoft hesabınla login ol
- "Create a new publisher" tıkla
- Publisher ID: `tw4` (package.json ile aynı olmalı)
- Display name: "TW4 Organization" 
- "Create" tıkla

### 2. Personal Access Token Al (3 dakika)  
```bash
# Web browser'da şu adrese git:
open https://dev.azure.com
```
- Sağ üst User Settings → Personal Access Tokens
- "New Token" tıkla
- Name: "nova-extension-publish"
- Scopes: **"Marketplace (Manage)"** seç (önemli!)
- "Create" tıkla
- Token'ı KOPYALA (bir daha gösterilmez!)

### 3. VSCE Login (Terminal'de)
```bash
cd /Users/mert/github/nova-lang/editor_support/vscode
npx vsce login tw4
# Token'ı yapıştır (görünmez ama yazılıyor)
```

### 4. PUBLISH ET! 
```bash
npx vsce publish
```

## 🎯 HIZLI YOL - Manuel Upload:
Eğer CLI ile sorun olursa:
```bash
open https://marketplace.visualstudio.com/manage
```
- "New Extension" → "Visual Studio Code" 
- `nova-language-extension-1.1.0.vsix` dosyasını sürükle bırak

## 🔍 Publish Sonrası:
Extension şu adreste yayınlanacak:
`https://marketplace.visualstudio.com/items?itemName=tw4.nova-language-extension`

## 🆕 Version 1.1.0 Yeni Özellikler:
- ✅ Advanced IDE features (formatting, refactoring)
- ✅ Project scaffolding templates
- ✅ Nova Light theme
- ✅ Enhanced auto-completion
- ✅ Better syntax validation
- ✅ Robust error handling

## 📦 Test Et:
```bash
# VS Code'da extension'ı ara ve yükle:
# Ctrl+Shift+X → "Nova Language Support" ara
```

Hazır! Bu adımları takip et ve extension publish edilmiş olacak! 🚀