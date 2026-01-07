const { spawn } = require("child_process");
const fs = require("fs");
const path = require("path");
const https = require("https");
const { createGunzip } = require("zlib");
const tar = require("tar");

const REPO = "nguyenphutrong/agentmap";
const BINARY_NAME = process.platform === "win32" ? "agentmap.exe" : "agentmap";

const PLATFORM_MAP = {
  darwin: "darwin",
  linux: "linux",
  win32: "windows",
};

const ARCH_MAP = {
  x64: "x86_64",
  arm64: "aarch64",
};

function getBinDir() {
  return path.join(__dirname, "..", "bin");
}

function getBinaryPath() {
  return path.join(getBinDir(), BINARY_NAME);
}

function getPlatformInfo() {
  const platform = PLATFORM_MAP[process.platform];
  const arch = ARCH_MAP[process.arch];

  if (!platform) {
    throw new Error(`Unsupported platform: ${process.platform}`);
  }
  if (!arch) {
    throw new Error(`Unsupported architecture: ${process.arch}`);
  }

  return { platform, arch };
}

function getDownloadUrl(version) {
  const { platform, arch } = getPlatformInfo();
  const filename = `agentmap-${platform}-${arch}.tar.gz`;
  return `https://github.com/${REPO}/releases/download/${version}/${filename}`;
}

async function getLatestVersion() {
  return new Promise((resolve, reject) => {
    const options = {
      hostname: "api.github.com",
      path: `/repos/${REPO}/releases/latest`,
      headers: {
        "User-Agent": "agentmap-npm",
      },
    };

    https
      .get(options, (res) => {
        let data = "";
        res.on("data", (chunk) => {
          data += chunk;
        });
        res.on("end", () => {
          try {
            const json = JSON.parse(data);
            resolve(json.tag_name);
          } catch (e) {
            reject(new Error("Failed to parse GitHub API response"));
          }
        });
      })
      .on("error", reject);
  });
}

function downloadFile(url) {
  return new Promise((resolve, reject) => {
    const request = (targetUrl) => {
      https
        .get(targetUrl, (res) => {
          if (res.statusCode === 301 || res.statusCode === 302) {
            request(res.headers.location);
            return;
          }

          if (res.statusCode !== 200) {
            reject(new Error(`Download failed with status ${res.statusCode}`));
            return;
          }

          resolve(res);
        })
        .on("error", reject);
    };

    request(url);
  });
}

async function downloadBinary(version) {
  const url = getDownloadUrl(version);
  const binDir = getBinDir();

  if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
  }

  console.log(`Downloading agentmap ${version}...`);

  const response = await downloadFile(url);

  await new Promise((resolve, reject) => {
    const gunzip = createGunzip();
    const extract = tar.extract({
      cwd: binDir,
      filter: (filePath) =>
        filePath === BINARY_NAME || filePath === "agentmap",
    });

    response
      .pipe(gunzip)
      .pipe(extract)
      .on("finish", resolve)
      .on("error", reject);
  });

  if (process.platform !== "win32") {
    fs.chmodSync(getBinaryPath(), 0o755);
  }

  fs.writeFileSync(path.join(binDir, ".version"), version);

  console.log(`agentmap ${version} installed successfully!`);
}

function needsDownload(version) {
  const binaryPath = getBinaryPath();
  const versionFile = path.join(getBinDir(), ".version");

  if (!fs.existsSync(binaryPath)) {
    return true;
  }

  if (!fs.existsSync(versionFile)) {
    return true;
  }

  const installedVersion = fs.readFileSync(versionFile, "utf-8").trim();
  return installedVersion !== version;
}

async function ensureBinary() {
  const version = await getLatestVersion();

  if (needsDownload(version)) {
    await downloadBinary(version);
  }

  return getBinaryPath();
}

function runBinary(args) {
  const binaryPath = getBinaryPath();

  if (!fs.existsSync(binaryPath)) {
    console.error(
      "Binary not found. Running postinstall to download binary..."
    );
    require("../scripts/postinstall.js");
    return;
  }

  const child = spawn(binaryPath, args, {
    stdio: "inherit",
    env: process.env,
  });

  child.on("error", (err) => {
    console.error(`Failed to start agentmap: ${err.message}`);
    process.exit(1);
  });

  child.on("close", (code) => {
    process.exit(code || 0);
  });
}

module.exports = {
  getBinaryPath,
  ensureBinary,
  runBinary,
  getLatestVersion,
  downloadBinary,
};
