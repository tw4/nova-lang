# Nova Extension Publishing Guide

## ✅ Extension Hazır!
- Package oluşturuldu: `nova-language-support-1.0.0.vsix`
- Boyut optimize edildi: 326KB (31 dosya)
- LICENSE dosyası eklendi
- .vscodeignore ile gereksiz dosyalar exclude edildi

## 📦 Local Installation Test
Extension'ı local olarak test etmek için:
```bash
code --install-extension nova-language-support-1.0.0.vsix
```

## 🌐 VS Code Marketplace'e Publish Etmek İçin:

### 1. Publisher Account Oluştur
1. https://marketplace.visualstudio.com/manage adresine git
2. Microsoft hesabınla giriş yap
3. "Create a new publisher" butonuna tıkla
4. Publisher ID girin (örn: "nova-team" veya "mertcanb")

### 2. Personal Access Token Oluştur
1. https://dev.azure.com adresine git
2. User Settings → Personal Access Tokens
3. "New Token" oluştur
4. Scopes: "Marketplace (Manage)" seç
5. Token'ı kaydet (bir daha gösterilmeyecek!)

### 3. VSCE ile Login
```bash
npx vsce login <publisher-id>
# Token'ı gir
```

### 4. Publish Et
```bash
npx vsce publish
```

## 🔄 Package.json'da Publisher ID Güncelle
Publish etmeden önce `package.json`'da publisher field'ını güncellemen gerekebilir:
```json
"publisher": "your-actual-publisher-id"
```

## 📋 Pre-Publish Checklist
- [x] Extension package edildi
- [x] LICENSE dosyası eklendi
- [x] .vscodeignore optimize edildi
- [ ] Publisher account oluşturuldu
- [ ] Personal Access Token oluşturuldu
- [ ] Package.json'da publisher ID güncellendi
- [ ] Extension publish edildi

## 🚀 Alternatif: Manual Upload
Web interface kullanarak da upload edebilirsin:
1. https://marketplace.visualstudio.com/manage
2. "New Extension" → "Visual Studio Code"
3. `.vsix` dosyasını drag & drop et

Extension hazır ve publish edilmeye hazır! 🎉