# 基本的なscaffoldコマンド
cargo loco generate scaffold User email:string name:string
cargo loco generate scaffold Post title:string content:text published:bool
cargo loco generate scaffold Category name:string description:string
cargo loco generate scaffold Tag name:string color:string
cargo loco generate scaffold Comment content:text

# 参照の追加
cargo loco generate migration AddUserRefToPosts user:references
cargo loco generate migration AddUserRefToComments user:references
cargo loco generate migration AddPostRefToComments post:references
cargo loco generate migration AddCategoryRefToPosts category:references

# 多対多のJoinテーブル
cargo loco generate migration CreateJoinTablePostsAndTags