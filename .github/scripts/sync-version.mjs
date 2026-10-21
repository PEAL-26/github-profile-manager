// Sincroniza a versão da aplicação nos ficheiros de projeto antes do build
// do release, para que os artefactos (MSI/EXE) e a tag reflitam a versão real.
//
// Uso: node .github/scripts/sync-version.mjs 0.2.1
import { readFileSync, writeFileSync } from "node:fs"
import { dirname, join } from "node:path"
import { fileURLToPath } from "node:url"

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "..", "..")
const version = process.argv[2]

if (!version || !/^\d+\.\d+\.\d+(-[\w.]+)?$/.test(version)) {
  console.error("Uso: node .github/scripts/sync-version.mjs <x.y.z>")
  process.exit(1)
}

const setJsonVersion = (relPath) => {
  const path = join(repoRoot, relPath)
  const data = JSON.parse(readFileSync(path, "utf8"))
  data.version = version
  writeFileSync(path, `${JSON.stringify(data, null, 2)}\n`)
  console.log(`${relPath} -> ${version}`)
}

setJsonVersion("project/package.json")
setJsonVersion("project/src-tauri/tauri.conf.json")

const cargoPath = join(repoRoot, "project/src-tauri/Cargo.toml")
const cargo = readFileSync(cargoPath, "utf8")
if (!/^version = "[^"]+"/m.test(cargo)) {
  console.error("project/src-tauri/Cargo.toml: campo version não encontrado")
  process.exit(1)
}
const synced = cargo.replace(/^version = "[^"]+"/m, `version = "${version}"`)
writeFileSync(cargoPath, synced)
console.log(`project/src-tauri/Cargo.toml -> ${version}`)
