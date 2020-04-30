# APLLO SQL (仮)

Immutable Schema （やその他の特徴（feature検討中））を備えたRDBMSのサーバ実装。

[![CircleCI](https://circleci.com/gh/darwin-education/apllo-immutable-schema-poc.svg?style=shield&circle-token=81302f3aa41b7c5eb7d6cc8d67d2cc355832e8be)](https://circleci.com/gh/darwin-education/apllo-immutable-schema-poc)
![MSRV](https://img.shields.io/badge/rustc-1.40+-lightgray.svg)

## 開発開始までの手順

TBD

## システム設計

### コンポーネント図

["Architecture of a Database System"](https://dsf.berkeley.edu/papers/fntdb07-architecture.pdf) の Fig. 1.1 をベースに記載。

<!-- ![]() 形式だと、GitHubが自前のCDNにキャッシュしようとしてなんか失敗する -->
<img src="http://drive.google.com/uc?export=view&id=1hlHuIgVHkGb_n8A8ZBKIyxtRBGqIDgfQ" alt="APLLO SQL (仮) コンポーネント図">

### 諸々ドキュメント

- [要求分析](https://docs.google.com/document/d/1J6_MWObo0VVo-ATrwALpoNUHBUbSvrxHV8XuBcs_tIM/edit)
- [要件定義](https://docs.google.com/document/d/1djtGGMope8eCJOMjDXl0DvjpUrwlGjHygUN8n0M-0WI/edit#heading=h.hhevn0icya3z)
- [仕様策定](https://docs.google.com/document/d/1yUgI-_hqPYiVBPYWQosuo3idVzAjbq29GgyS72N4SAs/edit)

- [Immutable Schema で解決したい課題](https://github.com/darwin-education/apllo/wiki/Immutable-Schema-000:-%E8%A7%A3%E6%B1%BA%E3%81%97%E3%81%9F%E3%81%84%E8%AA%B2%E9%A1%8C)
- [Immutable Schema 仕様書一覧](https://github.com/darwin-education/apllo/wiki/Immutable-Schema-100:-%E4%BB%95%E6%A7%98%E6%9B%B8%E4%B8%80%E8%A6%A7)
