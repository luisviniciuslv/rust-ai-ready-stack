# Solução: Erro de Protoc (Download Automático)

Se você encontrar erros de `protoc`, use o `Makefile` para baixar e configurar automaticamente um binário local (sem commitar executáveis no repositório).

---

## Passo único

### Windows (PowerShell)

Na raiz do projeto, execute:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\protoc-setup.ps1
```

### Linux / macOS

```bash
make protoc-setup
```

O alvo acima:

1. Baixa o `protoc` para `.tools/protoc`
2. Extrai o binário correto para o sistema operacional
3. Gera `.cargo/config.toml` com:

```toml
[env]
PROTOC = { value = ".tools/protoc/bin/protoc", relative = true }
```

No Windows, o valor será configurado automaticamente para `.tools/protoc/bin/protoc.exe`.

## Limpeza (opcional)

Para remover os artefatos locais de `protoc` e o `config.toml`:

### Windows (PowerShell)

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\protoc-clean.ps1
```

### Linux / macOS

```bash
make protoc-clean
```

## Nota

O `protoc` só precisa ser bootstrapado quando o ambiente exigir compilação de dependências que dependam dele.
