## polkacli: your gateway to the magical world of AssetHub

welcome to **polkacli**, the command-line tool you never knew you needed (or wanted) for interacting with AssetHub. think of it as your passport to the cryptoverse, only instead of stamps, you get NFTs, balances, and a healthy dose of blockchain jargon.  

### why should you care?

let's be real, you’re probably already neck-deep in crypto. so why not take the plunge and start managing your AssetHub NFTs, balances, and accounts with the finest cli on the market? whether you're minting a new NFT or just showing off your collection, **polkacli** is here to make sure you do it in style (or at least with less hassle).

### installation

assuming you know your way around a terminal, and have rust installed, just clone this repo and build it yourself. bc why wouldn't you?

```bash
git clone https://github.com/your-repo/polkacli.git
cd polkacli
cargo build --release
```

or, if you're feeling adventurous (and let's face it, you are), try installing via some fancy package manager. (coming soon™, probably.)

### usage

so you've installed **polkacli**. now what? here’s a quick rundown of what you can do. spoiler alert: it's a lot.

```bash
polkacli <COMMAND>
```

#### commands

- `mint-collection`: 
  - mint a shiny new NFT collection. your wallet will thank you later.
  - _requires the `nft` feature_.

- `mint-nft <COLLECTION_ID> <NFT_ID>`: 
  - mint an NFT within your collection. show the world (or maybe just your cat) what you've created.
  - _requires the `nft` feature_.

- `show-nft <COLLECTION_ID> <NFT_ID>`: 
  - retrieve and display the juicy details of a specific NFT. bc knowledge is power.

- `show-collection <COLLECTION_ID>`: 
  - get the lowdown on a specific collection. it’s like stalking, but for NFTs.

- `send <ADDRESS> <AMOUNT>`: 
  - send funds to an address. it’s like being a crypto philanthropist, minus the tax benefits.

- `set-account --mnemonic <MNEMONIC> | --secret-uri <SECRET_URI>`: 
  - configure your account. it's ez. it's in plaintext on your drive so don't use this for anything important plz

- `balance [ADDRESS]`: 
  - check the balance of the configured account, or any account you can dig up an address for. you might be richer than you think.

- `account <PUBLIC_KEY>`: 
  - get info about an account by its public key. bc who doesn’t like snooping on wallets?

### advanced usage

let’s be honest, if you're here, you’re probably going to figure out the advanced stuff on your own. but just in case:

1. **config file**: create a `.polkacli` config file in your home directory to store your account details, bc typing mnemonics every time is so 2020. or you can use the `set-account` cli to do it for you - duh.
2. **networking**: by default, **polkacli** connects to AssetHub. if you want to connect to another network, well, good luck.

### contributing

found a bug? have a feature request? want to write a README yourself? pull requests are welcome. just remember: if your code breaks my code, i will find you.

### disclaimer

**polkacli** comes with no warranty, express or implied. use at your own risk. if you lose your life savings while using this tool, that's on you, not me. (but seriously, maybe don’t do that.)

### license

licensed under apache 2.0, because open source should be free, but not in the "i can't make money off this" way.

happy hacking. or whatever it is you're doing.
