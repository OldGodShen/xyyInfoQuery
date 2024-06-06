import fs from 'fs';
import path from 'path';
import toml from 'toml';

const packageJsonPath = path.join(__dirname, 'package.json');
const cargoTomlPath = path.join(__dirname, 'src-tauri', 'Cargo.toml');
const tauriConfJsonPath = path.join(__dirname, 'src-tauri', 'tauri.conf.json');

const newVersion = process.argv[2];

if (!newVersion) {
  console.error('Please provide a new version.');
  process.exit(1);
}

// Update package.json
const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
packageJson.version = newVersion;
fs.writeFileSync(packageJsonPath, JSON.stringify(packageJson, null, 2));
console.log(`Updated package.json to version ${newVersion}`);

// Update Cargo.toml
const cargoTomlContent = fs.readFileSync(cargoTomlPath, 'utf8');
const cargoToml = toml.parse(cargoTomlContent);
cargoToml.package.version = newVersion;
const newCargoTomlContent = cargoTomlContent.replace(
  /version\s*=\s*".*"/,
  `version = "${newVersion}"`
);
fs.writeFileSync(cargoTomlPath, newCargoTomlContent);
console.log(`Updated Cargo.toml to version ${newVersion}`);

// Update tauri.conf.json
const tauriConfJson = JSON.parse(fs.readFileSync(tauriConfJsonPath, 'utf8'));
tauriConfJson.version = newVersion;
fs.writeFileSync(tauriConfJsonPath, JSON.stringify(tauriConfJson, null, 2));
console.log(`Updated tauri.conf.json to version ${newVersion}`);
