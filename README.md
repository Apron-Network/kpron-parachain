# Kpron Node

The Apron parachain, will deploy on kusama, powered by substrate, cumulus and more... 

## Build & Run

Follow these steps to prepare a local Substrate development environment :hammer_and_wrench:

### Setup of Machine

If necessary, refer to the setup instructions at the
[Substrate Developer Hub](https://substrate.dev/docs/en/knowledgebase/getting-started/#manual-installation).

### Build

Once the development environment is set up, build the parachain node template. This command will
build the
[Wasm Runtime](https://substrate.dev/docs/en/knowledgebase/advanced/executor#wasm-execution) and
[native node](https://substrate.dev/docs/en/knowledgebase/advanced/executor#native-execution) code:

```bash
cargo build --release
```

## Connect a Collator Node to a Relay Chain 

### Local Relay Chain Testnet

To operate a parathread or parachain, you _must_ connect to a relay chain.

#### Relay Chain Network (Validators)

Clone and build Polkadot (**at the correct commit for your relay chain**):
```bash
# Get a fresh clone, or `cd` to where you have polkadot already:
git clone -b <YOUR RELAY CHAIN BRANCH OR RELEASE TAG> --depth 1 https://github.com:paritytech/polkadot.git
cd polkadot
cargo build --release
```

##### Generaete the chainspec

> NOTE: this file _must_ be generated on a _single node_ and then the file shared with all nodes!
> Other nodes _cannot_ generate it due to possible non-determinism. 

```bash
./target/release/polkadot build-spec \
--chain westend-local \
--raw \
--disable-default-bootnode \
> westend_local.json
```

##### Start Relay Chain Node(s)

You should have a minimum of 2 running full _validator_ nodes on your relay chain per parachain/thread
collator you intend to connect!

From the Polkadot working directory:
```bash
# Start Relay `Alice` node
./target/release/polkadot \
--chain ./westend_local.json \
-d /tmp/relay/alice \
--validator \
--alice \
--port 50555
```

Open a new terminal, same directory: 

```bash
# Start Relay `Alice` node
./target/release/polkadot \
--chain ./westend_local.json \
-d /tmp/relay/bob \
--validator \
--bob \
--port 50556
```
Add more nodes as needed, with non-conflicting ports, DB directories, and validator keys
(`--charlie`, `--dave`, etc.).

##### Reserve a ParaID

To connect to a relay chain, you must first _reserve a `ParaId` for your parathread that will 
become a parachain. To do this, you _must_ have currency available on an account on that network
in sufficient amount to reserve an ID. This is 20 "units" on the testnets, check for the amount
on your relay chain. The relay chain will increment starting at `2000` for all chains connecting
that are not "systems parachains" that use a different method to obtain a `ParaId`.

The easiest way to reserve your `ParaId` this is via the
[Polkadot Apps UI](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/parachains/parathreads)
under the Parachains -> Parathreads tab and use the `+ ParaID` button.

> You will need to connect to a _relay chain node_ to submit this extrinsic!
> In testnets, your ParaId will be 2000 for your first parathread registration.

In this example flow, we will use the **`Charlie` development account** where we have funds available. 
Once you submit this extrinsic successfully, you can start your collators.

### Parachain Network

#### Export the Parachain Genesis and Runtime

The files you will need to register we will generate in a `./resources` folder, to build them because
you modified the code you can use the following commands:

```bash
# Build the parachain node (from it's top level dir)
cargo build --release

# Place to store files we need
mkdir resources 

# Build the Chain spec
./target/release/Kpron-node build-spec \
--disable-default-bootnode > ./resources/template-local-plain.json

# Build the raw file
./target/release/Kpron-node build-spec \
--chain=./resources/template-local-plain.json \
--raw --disable-default-bootnode > ./resources/template-local.json


# Export genesis state to `./resources files
# Assumes ParaId = 2000 . Change as needed.
./target/release/Kpron-node export-genesis-state --parachain-id 2000 > ./resources/para-2000-genesis
# export runtime wasm
./target/release/Kpron-node export-genesis-wasm > ./resources/para-2000-wasm
```

> Note: we have set the `para_ID = 2000` here, this _must_ be unique for all parathreads/chains on the
> relay chain you register with. You _must_ reserve this first on the relay chain!

#### Start Parachain Nodes (Collators)

From the parachain template working directory:

```bash
# NOTE: this command assumes the chain spec is in a directory named `polkadot`
# that is at the same level of the template working directory. Change as needed.
#
# It also assumes a ParaId oof 2000. Change as needed.
./target/release/Kpron-node \
-d /tmp/parachain/alice \
--collator \
--alice \
--force-authoring \
--ws-port 9945 \
--parachain-id 2000 \
-- \
--execution wasm \
--chain ../polkadot/westend_local.json
```

#### Register on the Relay with `sudo`

In order to produce blocks you will need to register the parachain as detailed in the
[Substrate Cumulus Worship](https://substrate.dev/cumulus-workshop/#/en/3-parachains/2-register)
by going to:

`Developer -> sudo -> paraSudoWrapper -> sudoScheduleParaInitialize(id, genesis)`

Ensure you set the `ParaId to 2000` and the `parachain: Bool to Yes`.

The files you will need are in the `./resources` folder, you just created.

> Note : When registering to the public Rococo testnet, ensure you set a **unique** 
> `para_id` > 1000, below 1000 is reserved _exclusively_ for system parachains.

#### Restart the Parachain (Collator) and Wait...

The collator node may need to be restarted to get it functioning as expected. After a 
[new era](https://wiki.polkadot.network/docs/en/glossary#era) starts on the relay chain,
your parachain will come online. Once this happens, you should see the collator start
reporting _parachian_ blocks:

```bash
2021-04-01 16:31:06 [Relaychain] ✨ Imported #243 (0x46d8…f394)    
2021-04-01 16:31:06 [Relaychain] 👴 Applying authority set change scheduled at block #191    
2021-04-01 16:31:06 [Relaychain] 👴 Applying GRANDPA set change to new set [(Public(88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee (5FA9nQDV...)), 1), (Public(d17c2d7823ebf260fd138f2d7e27d114c0145d968b5ff5006125f2414fadae69 (5GoNkf6W...)), 1)]    
2021-04-01 16:31:06 [Relaychain] 👴 Imported justification for block #191 that triggers command Changing authorities, signaling voter.    
2021-04-01 16:31:06 [Parachain] Starting collation. relay_parent=0x46d87d4b55ffcd2d2dde3ee2459524c41da48ac970fb1448feaa26777b14f394 at=0x85c655663ad333b1508d0e4a373e86c08eb5b5353a3eef532a572af6395c45be
2021-04-01 16:31:06 [Parachain] 🙌 Starting consensus session on top of parent 0x85c655663ad333b1508d0e4a373e86c08eb5b5353a3eef532a572af6395c45be    
2021-04-01 16:31:06 [Parachain] 🎁 Prepared block for proposing at 91 [hash: 0x078560513ac1862fed0caf5726b7ca024c2af6a28861c6c69776b61fcf5d3e1f; parent_hash: 0x85c6…45be; extrinsics (2): [0x8909…1c6c, 0x12ac…5583]]    
2021-04-01 16:31:06 [Parachain] Produced proof-of-validity candidate. pov_hash=0x836cd0d72bf587343cdd5d4f8631ceb9b863faaa5e878498f833c7f656d05f71 block_hash=0x078560513ac1862fed0caf5726b7ca024c2af6a28861c6c69776b61fcf5d3e1f
2021-04-01 16:31:06 [Parachain] ✨ Imported #91 (0x0785…3e1f)    
2021-04-01 16:31:09 [Relaychain] 💤 Idle (2 peers), best: #243 (0x46d8…f394), finalized #192 (0x9fb4…4b28), ⬇ 1.0kiB/s ⬆ 3.2kiB/s    
2021-04-01 16:31:09 [Parachain] 💤 Idle (0 peers), best: #90 (0x85c6…45be), finalized #64 (0x10af…4ede), ⬇ 1.1kiB/s ⬆ 1.0kiB/s    
2021-04-01 16:31:12 [Relaychain] ✨ Imported #244 (0xe861…d99d)    
2021-04-01 16:31:14 [Relaychain] 💤 Idle (2 peers), best: #244 (0xe861…d99d), finalized #193 (0x9225…85f1), ⬇ 2.0kiB/s ⬆ 1.6kiB/s    
2021-04-01 16:31:14 [Parachain] 💤 Idle (0 peers), best: #90 (0x85c6…45be), finalized #65 (0xdd20…d44a), ⬇ 1.6kiB/s ⬆ 1.4kiB/s    
``` 

> Note the delay here! It may take some time for your relaychain to enter a new era.

