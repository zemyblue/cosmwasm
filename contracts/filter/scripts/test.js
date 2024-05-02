#!/usr/bin/env -S yarn node

const { SigningFinschiaClient, makeLinkPath } = require("@finschia/finschia");
const { DirectSecp256k1HdWallet } = require("@cosmjs/proto-signing");
const { calculateFee, GasPrice } = require("@cosmjs/stargate");
const fs = require("fs");
const { assert } = require("console");

const endpoint = "http://localhost:26657";
const alice = {
    mnemonic: "mind flame tobacco sense move hammer drift crime ring globe art gaze cinnamon helmet cruise special produce notable negative wait path scrap recall have",
    address0: "link146asaycmtydq45kxc8evntqfgepagygelel00h",
    address1: "link1aaffxdz4dwcnjzumjm7h89yjw5c5wul88zvzuu",
    address2: "link1ey0w0xj9v48vk82ht6mhqdlh9wqkx8enkpjwpr",
    address3: "link1dfyywjglcfptn72axxhsslpy8ep6wq7wujasma",
}

class FilterClient {
    constructor(client, codeId) {
        this.client = client;
        this.codeId = codeId;
        this.gasPrice = GasPrice.fromString("0.025cony");
    }

    async instantiateContract(sender) {
        const instantiateFee = calculateFee(500_000, this.gasPrice);
        const instRes = await this.client.instantiate(
            sender,
            this.codeId,
            {},
            "filter instantiate",
            instantiateFee,
            {
                memo: "Create a Test Filter",
                admin: sender,
            }
        );
        console.info(`Contract instantiated at ${instRes.contractAddress} in ${instRes.transactionHash}`);

        return instRes.contractAddress;
    }

    async callSendNFT(contractAddress, sender) {
        const fee = calculateFee(200_000, this.gasPrice);
        const msg = {
            call_send_nft: {}
        }
        try {
            const res = await this.client.execute(sender, contractAddress, msg, fee);
            console.info(`CallSendNFT txHash: ${res.transactionHash}`);

            return res.transactionHash;
        } catch (error) {
            console.info(`CallSendNFT err: ${error}`);
            return error;
        }
    }
}

async function deployContract(client, sender, wasmData) {
    const gasPrice = GasPrice.fromString("0.025cony");
    const uploadFee = calculateFee(1_500_000, gasPrice);
    const uploadReceipt = await client.upload(sender, wasmData, uploadFee, "Upload standard contract");
    console.info(`Upload succeeded. Receipt: ${JSON.stringify(uploadReceipt)}`);

    return uploadReceipt.codeId;
}

async function main() {
    console.info("test");
    const wallet = await DirectSecp256k1HdWallet.fromMnemonic(alice.mnemonic, {
        hdPaths: [makeLinkPath(0), makeLinkPath(1), makeLinkPath(2), makeLinkPath(3)],
        prefix: "link",
    });
    const client = await SigningFinschiaClient.connectWithSigner(endpoint, wallet);

    // deploy
    const wasm = fs.readFileSync(__dirname + "/../artifacts/filter.wasm");
    const codeId = await deployContract(client, alice.address0, wasm);

    const fileterClient = new FilterClient(client, codeId);

    // instantiate
    const contractAddress = await fileterClient.instantiateContract(alice.address0);

    // execute CallSendNFT
    await fileterClient.callSendNFT(contractAddress, alice.address0);
}

main().then(
    () => {
        console.info("all done");
        process.exit(0);
    },
    (error) => {
        console.error(error);
        process.exit(1);
    }
)
