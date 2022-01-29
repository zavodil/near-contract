NEAR Smart Contract Boilerplate
======

- Basic smart contract structure
- Jest sim tests
- Build local/docker binaries
- Build & deploy scripts

HOW TO USE
======
Create new repo && download this boilerplate to target folder:

```
wget https://github.com/zavodil/near-contract/archive/refs/heads/main.zip -O "near-contract-master.zip" && unzip ./"near-contract-master.zip" -d $PWD && rm ./"near-contract-master.zip" && mv -v $PWD/near-contract-main/* $PWD && rm -rf near-contract-main
```


BUILD DOCKER ON M1:
===
Run docker buildx `contract-builder`
```
 ~/projects/near-sdk-rs/contract-builder/build_m1.sh
 ~/projects/near-sdk-rs/contract-builder/run_m1.sh
 
 ./build_docker_m1.sh
```

