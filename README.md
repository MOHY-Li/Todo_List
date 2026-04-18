# Todo List (Dioxus Desktop)

一个专注「昨天 / 今天 / 明天」三日规划的桌面 Todo 应用，使用 Rust + Dioxus 构建。

## 功能特性

- 三日规划视图：昨天 / 今天 / 明天
- 任务优先级：低 / 中 / 高（带颜色与图标）
- 顶部优先级快速筛选：全部 / 低 / 中 / 高
- 分类管理：
  - 新建任务可填写分类
  - 支持复用历史分类（常用标签）
  - 侧边栏按分类过滤
- 任务搜索（防抖输入）
- 任务编辑、完成切换、删除
- 本地持久化存储（自动保存到用户本地目录）

## 技术栈

- Rust (edition 2021)
- Dioxus 0.7 (desktop)
- Tailwind CSS（CDN 引入）
- Serde / Serde JSON
- Chrono / Tokio

## 本地运行

```bash
cargo run
```

## 开发检查

```bash
cargo check
```

## 数据存储位置

应用数据文件默认保存在：

- macOS: `~/Library/Application Support/dioxus-todo/todos.json`

## 项目结构

```text
src/
  components/      # UI 组件（Sidebar, TodoList, AddForm 等）
  icons/           # SVG 图标组件
  models/          # 数据模型（Todo, Priority, Tab 等）
  storage/         # 本地存储读写
  main.rs          # 应用入口与主布局
assets/
  app.css          # 额外样式
```

## 未来计划

- 键盘快捷键
- 拖拽排序
- 导入/导出
- 多主题配色

## License

MIT
