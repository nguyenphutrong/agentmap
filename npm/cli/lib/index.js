const { spawn } = require("child_process");
const fs = require("fs");
const path = require("path");

const BINARY_NAME = process.platform === "win32" ? "agentlens.exe" : "agentlens";

const PLATFORM_PACKAGES = {
  "darwin-arm64": "@agentlens/darwin-arm64",
  "darwin-x64": "@agentlens/darwin-x64",
  "linux-arm64": "@agentlens/linux-arm64",
  "linux-x64": "@agentlens/linux-x64",
  "win32-x64": "@agentlens/win32-x64",
};

function getPlatformPackage() {
  const platform = process.platform;
  const arch = process.arch === "arm64" ? "arm64" : "x64";
  const key = `${platform}-${arch}`;
  return PLATFORM_PACKAGES[key];
}

function getBinaryPath() {
  const platformPackage = getPlatformPackage();

  if (!platformPackage) {
    throw new Error(
      `Unsupported platform: ${process.platform}-${process.arch}`
    );
  }

  try {
    const packagePath = require.resolve(`${platformPackage}/package.json`);
    const binPath = path.join(path.dirname(packagePath), "bin", BINARY_NAME);

    if (fs.existsSync(binPath)) {
      return binPath;
    }
  } catch (e) {}

  const localBinPath = path.join(__dirname, "..", "bin", BINARY_NAME);
  if (fs.existsSync(localBinPath)) {
    return localBinPath;
  }

  throw new Error(
    `agentlens binary not found. Please reinstall @agentlens/cli or install manually:\n` +
      `  cargo install agentlens\n` +
      `  brew install nguyenphutrong/tap/agentlens`
  );
}

function runBinary(args) {
  const binaryPath = getBinaryPath();

  const child = spawn(binaryPath, args, {
    stdio: "inherit",
    env: process.env,
  });

  child.on("error", (err) => {
    console.error(`Failed to start agentlens: ${err.message}`);
    process.exit(1);
  });

  child.on("close", (code) => {
    process.exit(code || 0);
  });
}

module.exports = {
  getBinaryPath,
  runBinary,
};
