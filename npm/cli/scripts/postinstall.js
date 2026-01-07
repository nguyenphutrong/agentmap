#!/usr/bin/env node

const { ensureBinary } = require("../lib/index.js");

ensureBinary()
  .then(() => {
    console.log("agentmap is ready to use!");
  })
  .catch((err) => {
    console.error("Failed to install agentmap binary:", err.message);
    console.error("");
    console.error("You can install manually:");
    console.error("  cargo install agentmap");
    console.error("");
    console.error("Or download from:");
    console.error("  https://github.com/nguyenphutrong/agentmap/releases");
    process.exit(1);
  });
