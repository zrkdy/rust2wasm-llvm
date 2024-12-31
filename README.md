# rust2wasm-llvm
# Rust 转 WebAssembly

**本测试环境**: Windows 11, Rustc 1.83.0

## 步骤

### 1. 创建 Rust 项目
```bash
cargo new my_project --lib
```

### 2. 配置 Cargo.toml 文件
在 `Cargo.toml` 文件中添加相应的依赖和配置。

### 3. 编译为 .wasm 文件

#### 方法 1: 使用 `wasm-pack`
在 `pkg` 文件夹下生成对应文件，可通过 HTML 文件在浏览器中验证。
```bash
wasm-pack build --target web
```

#### 方法 2: 使用 `wasm32-unknown-unknown`
在 `target` 文件夹下生成对应文件。
```bash
cargo build --target wasm32-unknown-unknown --release
```

### 4. 比较两种生成的 .wasm 文件

#### 差异分析
- **`cargo build --target wasm32-unknown-unknown --release`**:
  - 生成纯粹的 WebAssembly 文件，不包含 `wasm-bindgen` 的元数据。
- **`wasm-pack build --target web`**:
  - 生成经过 `wasm-bindgen` 处理的 WebAssembly 文件，包含额外的元数据和胶水代码。

#### 使用场景
- 如果需要手动处理 WebAssembly 模块，可以选择 `cargo build`。
- 如果需要直接在 Web 项目中使用，则建议使用 `wasm-pack`。

### 5. 文件格式转换
使用 `wasm2wat` 和 `wat2wasm` 工具对 `.wasm` 和 `.wat` 文件进行互转。

- 将 `.wasm` 转为 `.wat` 文件格式，以查看文件结构：
  ```bash
  wasm2wat input.wasm -o output.wat
  ```
- 将 `.wat` 文件转回 `.wasm` 文件：
  ```bash
  wat2wasm output.wat -o output.wasm
  
