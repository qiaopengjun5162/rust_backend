# rust_backend

## Explain main code

这段代码是一个简单的 Actix-Web 服务器，用于处理与披萨相关的请求。

首先，代码导入了所需的依赖项和模块。然后，定义了三个路由处理函数： `get_pizzas` 、 `buy_pizza` 和 `update_pizza` 。这些函数分别处理获取所有比萨、购买比萨和更新比萨的请求。

 `get_pizzas` 函数通过调用 `Database::get_all_pizzas` 方法从数据库中获取所有比萨，并将结果作为 JSON 响应返回。如果没有找到比萨，将返回一个 `NoPizzasFound` 错误。

 `buy_pizza` 函数首先对传入的 JSON 请求体进行验证，验证通过后将生成一个新的 UUID，并使用该 UUID 和比萨名称创建一个新的比萨对象。然后，调用 `Database::add_pizza` 方法将新的比萨添加到数据库中，并将结果作为 JSON 响应返回。如果添加比萨失败，将返回一个 `PizzaCreationFailure` 错误。

 `update_pizza` 函数从 URL 路径中获取比萨的 UUID，并调用 `Database::update_pizza` 方法更新该比萨。更新成功后，将返回更新后的比萨对象作为 JSON 响应。如果找不到要更新的比萨，将返回一个 `NoSuchPizzaFound` 错误。

最后， `main` 函数初始化数据库并创建一个 HTTP 服务器。服务器配置了路由处理函数，并绑定到本地地址的 8080 端口上。然后，服务器开始运行并监听来自客户端的请求。

代码的主要目的是提供一个简单的比萨订购系统的后端服务。

## 实操

```shell
cargo new rust_backend
cd rust_backend/
cargo add actix-web json serde surrealdb uuid validator async-trait derive_more
cargo add serde --features derive
cargo add validator --features derive
cargo run
cargo watch -x run -c -q
brew install surrealdb/tap/surreal
surreal version
ls -l
surreal start file:pizzashop.dd --user root --password root
cargo add surrealdb
```

- <https://surrealdb.com/>
- <https://crates.io/crates/surrealdb>
- <https://docs.surrealdb.com/docs/introduction/start/>
- <https://surrealdb.com/install>
