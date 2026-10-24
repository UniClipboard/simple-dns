# UniClipboard DNS 修复

基线：balliegojr/simple-dns `58e2ed41f2ec4377ea2ab1802baf7456acb1f13b`，包版本 0.12.0。

依据 RFC 6891 §6.1.3–6.1.4，修正 OPT TTL 中扩展 RCODE 与版本的掩码及读写位移；低 16 位保留标志不参与错误码解析。同步修正文档报文。

`simple-dns/tests/edns_wire.rs` 覆盖 Windows 原始成功响应、保留标志与 DO 位、真实 BADVERS、版本、固定 wire 编码及无 OPT 行为。保留现有 RCODE::Reserved API，不扩展未知错误码表达能力。

上游发布完整修复后移除此 fork 固定来源。没有修改应用 DNS 错误处理、网络设置或重试策略。
