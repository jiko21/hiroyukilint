# hiroyukilint
[![Rust](https://github.com/jiko21/hiroyukilint/actions/workflows/ci-test.yml/badge.svg)](https://github.com/jiko21/hiroyukilint/actions/workflows/ci-test.yml)
[![release](https://github.com/jiko21/hiroyukilint/actions/workflows/ci-releaser.yml/badge.svg)](https://github.com/jiko21/hiroyukilint/actions/workflows/ci-releaser.yml)

論破されそうな語句があれば指摘するlinter
## 概要
> 概要を概要であると見抜ける人でないと(`hiroyukilint`を使うのは)難しい

`明らかに`や`必ず`のような、ひろゆきに論破されそうな単語が文章中に存在すると以下のように警告してくれます

![実行結果](./assets/result.png)

## 実行方法
> 実行方法を実行方法であると見抜ける人でないと(`hiroyukilint`を使うのは)難しい

```bash
hiroyukilint <チェックしたいファイル>
```
