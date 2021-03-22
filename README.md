# wasm_contract_gen

生成wasm合约 SDK
```
├── Cargo.toml.bak          // cargo workspace用于开发
├── README.md               // 项目readme
├── congen                  // 用于合约项目的创建
├── contract_test           // wasm2tc合约测试
├── rust-toolchain          // 限定rust版本为nightly
├── tests                   // 使用rust std标准库对wasm2tc测试
├── wasm2ct                 // wasm to contract wasm合约生成sdk
├── wasm_mid                // wasm与虚拟机的中间层,主要为虚拟机为wasm合约提供的接口
└── wasm_std                // wasm2ct的标准库
```

相关项目:
- [boyachain](http://10.0.0.20:3680/RegChain/boyachain)
- [regchainvm](http://10.0.0.20:3680/haojk/regchainvm)
- [new_wasm_sdk](http://10.0.0.20:3680/liusk/new_wasm_sdk)


当前合约与链完整测试流程:
## 1.下载编译[boyachain](http://10.0.0.20:3680/BOYAblockchain/boyachain)
## 2.启动boyachain(单独启动模式)
```
./boyachain clean
./boyachain init
./boyachain node bootstrap
./boyachain node --debug=true
```
具体命令以[boyachain](http://10.0.0.20:3680/BOYAblockchain/boyachain)为主
## 3.下载wasm虚拟机[regchainvm](http://10.0.0.20:3680/haojk/regchainvm)
## 4.在目录下启动wasm虚拟机
`cargo run --bin web_service --features "web_service"`
## 5.编译部署调用合约,以[calculator](http://10.0.0.20:3680/liusk/refactor_sdk/src/branch/master/contract_test/calculator)为例

### (0)调用blockchain start
> Url:   http://127.0.0.1:5678/vm/config/start
>
> Methods	:	GET
>
> Parameters	:无
成功返回:
```json
{
    "result": "success"
}
```

### (1)编译合约
```bash
cd contract_test/calculator
./build.sh
```

### (2)获取合约二进制文件
```bash
python3 readhex.py
```

### (3)调用 部署合约接口
> Url:   http://127.0.0.1:5554/vm/exec_tx
>
> Methods	:	GET
>
> Parameters	:(json格式)
```json
{
"tx": {
    "hash": "0x0000000000000000000000000000000000000000000000000000000000000000",
    "nonce": "0",
    "block_hash": "0x0000000000000000000000000000000000000000000000000000000000000000",
    "block_number": "0",
    "transaction_index": "0",
    "from": "0x0000000000000000000000000000000000000000",
    "to": null,
    "value": "0",
    "gas": "0",
    "gas_price": "0",
    "input": "", // 合约中constructor函数参数
    "code": ""   // 合约16进制可通过http://10.0.0.20:3680/liusk/new_wasm_sdk/src/branch/master/readhex.py获取
    },
"blockheader": {
    "coinbase": "0x0000000000000000000000000000000000000000",
    "timestamp": 0,
    "number": "0",
    "difficulty": "0",
    "gas_limit": "0"
    }
}
```
客户端返回:
```json
{
    "writeset": {
        "accounts": {
            "0x5943996084de04ffa21570dd6c1b91ea35af51ba": {
                "code": "0x..........", 
                "storage": {}
            }
        },
        "logs": []
    },
    "return_val": "0x"
}
```

### (4)将`writeset`写入链中
> Url:   http://127.0.0.1:5678/vm/test/edit_account
>
> Methods	:	GET
>
> Parameters	:(json格式)
```json
{
        "accounts": {
            "0x5943996084de04ffa21570dd6c1b91ea35af51ba": {
                "code": "0x......",
                "storage": {}
            }
        },
        "logs": []
    }    
```
成功返回:
```json
{
    "res": "success"
}
```

### (5)调用 调用合约接口
> Url:   http://127.0.0.1:5554/vm/exec_tx
>
> Methods	:	GET
>
> Parameters	:(json格式)
```json
{
    "tx": {
        "hash": "0x0000000000000000000000000000000000000000000000000000000000000000",
        "nonce": "0",
        "block_hash": "0x0000000000000000000000000000000000000000000000000000000000000000",
        "block_number": "0",
        "transaction_index": "0",
        "from": "0x0000000000000000000000000000000000000000",
        "to": "0x5943996084de04ffa21570dd6c1b91ea35af51ba",
        "value": "0",
        "gas": "0",
        "gas_price": "0",
        "input": "", // 通过ethabi编码的函数名及参数
        "code": null
    },
    "blockheader": {
        "coinbase": "0x0000000000000000000000000000000000000000",
        "timestamp": 0,
        "number": "0",
        "difficulty": "0",
        "gas_limit": "0"
    }
}   
```
成功返回:
```json
{
    "writeset": {
        "accounts": {},
        "logs": []
    },
    "return_val": "" //ethabi编码的函数调用返回
}
```