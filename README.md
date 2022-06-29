# HAHOE

유네스코지정 세계유산, 한국의 미와 전통이 살아있는 역사 마을.
하회 마을의 지형을 그래픽에 담다.

## 사용법

### 실행환경

target: wasm32-unknown-unknown  
  
- node v16 LTS
- volta v1.0.7
- yarn v1.22.18
- cargo v1.58.0
- cargo-watch v8.1.1
- wasm-pack v0.10.3

### 프로젝트 구조

```bash
$ tree
hahoe
├── Cargo.lock
├── Cargo.toml
├── package.json
├── yarn.lock
├── README.md
├── core
│   ├── Cargo.toml
│   └── src
├── gui
│   ├── Cargo.toml
│   └── src
├── hahoe.code-workspace
├── public
    ├── index.html
    └── index.js

```

### 실행방법

```bash
$ rustup target add wasm32-unknown-unknown
// target으로 사용할 wasm32-unknown-unknown 구성 (Tier 2 target)

$ cd gui && wasm-pack build
// gui로 이동해 wasm pkg 생성

$ yarn dev
// 최상위 작업 디렉토리에서 webpack-dev-server 실행
```
