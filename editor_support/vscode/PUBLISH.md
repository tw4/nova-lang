# Nova VS Code Extension - Marketplace Yayımlama Kılavuzu

## Ön Hazırlık

### 1. Visual Studio Marketplace Hesabı Oluştur

1. https://marketplace.visualstudio.com/ adresine git
2. "Publish extensions" linkine tıkla
3. Microsoft/Azure hesabınla giriş yap
4. Yeni bir Publisher profili oluştur:
   - Publisher Name: `tw4`
   - Display Name: `tw4`

### 2. Personal Access Token (PAT) Oluştur

1. https://dev.azure.com/ adresine git
2. User Settings > Personal Access Tokens
3. "New Token" oluştur:
   - Name: `vscode-extensions`
   - Organization: `All accessible organizations`
   - Scopes: `Marketplace > Manage`

### 3. VSCE (VS Code Extension Manager) Yükle

```bash
npm install -g vsce
```

## Yayımlama Süreci

### 1. Extension'ı Package'la

```bash
cd editor_support/vscode
vsce package
```

Bu komut `nova-language-0.1.0.vsix` dosyası oluşturur.

### 2. Login ve Publish

```bash
# Publisher olarak login ol
vsce login tw4

# Extension'ı publish et
vsce publish
```

### 3. Manual Upload (Alternatif)

Eğer komut satırı publish işlemi çalışmazsa:

1. https://marketplace.visualstudio.com/manage/publishers/tw4 adresine git
2. "New extension" > "Visual Studio Code"
3. `.vsix` dosyasını upload et

## Güncelleme Süreci

Version'ı güncellemek için:

```bash
# Version'ı otomatik artır ve publish et
vsce publish patch  # 0.1.0 -> 0.1.1
vsce publish minor  # 0.1.0 -> 0.2.0
vsce publish major  # 0.1.0 -> 1.0.0

# Veya manuel version belirt
vsce publish 0.2.0
```

## Önemli Notlar

- Extension name `nova-language` unique olmalı
- Publisher name `tw4` kullanıldı
- Repository: https://github.com/tw4/nova-lang
- License: MIT
- Tüm referanslar senin adına (tw4)

## Test Etme

Local test için:

```bash
# Package oluştur
vsce package

# VS Code'da test et
code --install-extension nova-language-0.1.0.vsix
```

## Marketplace Linki

Yayımlandıktan sonra extension şu adreste bulunacak:
https://marketplace.visualstudio.com/items?itemName=tw4.nova-language