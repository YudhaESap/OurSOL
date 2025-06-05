const connectBtn = document.getElementById("connect-btn");
const uploadBtn = document.getElementById("upload-btn");
const mintBtn = document.getElementById("mint-btn");
const ipfsUrlP = document.getElementById("ipfs-url");

let provider = null;
let publicKey = null;

connectBtn.onclick = async () => {
  if ("solana" in window) {
    provider = window.solana;
    try {
      const resp = await provider.connect();
      publicKey = resp.publicKey.toString();
      connectBtn.innerText = `Connected: ${publicKey.slice(0, 8)}...`;
    } catch (err) {
      alert("Wallet connect failed.");
    }
  } else {
    alert("Install Phantom Wallet first.");
  }
};

uploadBtn.onclick = async () => {
  const text = document.getElementById("soul-input").value.trim();
  if (!text) return alert("Enter your soul text first.");

  const NFT_STORAGE_KEY = "YOUR_NFT_STORAGE_API_KEY"; // replace with your key

  const res = await fetch("https://api.nft.storage/upload", {
    method: "POST",
    headers: {
      Authorization: `Bearer ${NFT_STORAGE_KEY}`,
      "Content-Type": "text/plain",
    },
    body: text,
  });

  const data = await res.json();
  if (data.ok) {
    const cid = data.value.cid;
    const url = `https://ipfs.io/ipfs/${cid}`;
    ipfsUrlP.innerText = `✅ Uploaded: ${url}`;
    mintBtn.disabled = false; // Enable minting later
  } else {
    alert("Upload failed.");
  }
};