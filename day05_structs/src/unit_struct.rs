// 定义不同的状态类型（空结构体）
#[derive(Debug)]
struct Draft;
#[derive(Debug)]
struct Published;

// 定义 trait，代表“可以查看内容”
trait ShowContent {
    fn content(&self) -> &str;
}

// 主文档结构，带状态泛型
#[derive(Debug)]
struct Document<State> {
    content: String,
    state: State,
}

// 实现创建草稿状态的方法
impl Document<Draft> {
    fn new(content: &str) -> Self {
        Document {
            content: content.to_string(),
            state: Draft,
        }
    }

    // 发布：从 Draft → Published 状态
    fn publish(self) -> Document<Published> {
        Document {
            content: self.content,
            state: Published,
        }
    }
}

// 只有 Published 状态才能查看内容
impl ShowContent for Document<Published> {
    fn content(&self) -> &str {
        &self.content
    }
}

pub fn unit_struct() {
    let draft = Document::new("Rust is awesome!");

    // draft.content(); // ❌ 编译错误，草稿不能查看内容

    let published = draft.publish();

    // ✅ 只有发布后的文档才能查看内容
    println!("内容: {}", published.content());
}