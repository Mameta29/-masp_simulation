use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

// ノート構造体の定義。資産のタイプと量を保持
#[derive(Debug, Clone, Hash)]
struct Note {
    asset_type: String,
    amount: u64,
}

impl Note {
    // ノートのハッシュ値を計算して、簡易的なコミットメントを生成
    fn commit(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

// トランザクション構造体の定義。
#[derive(Debug)]
struct Transaction {
    from_commit: u64, // 送信元ノートのコミットメント
    to: String,       // 受取人のユーザーID
    to_commit: u64,   // 受取人のノートコミットメント
}

impl Transaction {
    // トランザクションの新規作成。送信元と受取人のノートからコミットメントを計算します。
    fn new(from: &Note, to: &str, to_note: &Note) -> Self {
        Transaction {
            from_commit: from.commit(),
            to: to.to_string(),
            to_commit: to_note.commit(),
        }
    }
}

// ユーザー構造体の定義。ユーザーIDと所有するノートのリストを保持
#[derive(Debug)]
struct User {
    id: String,
    notes: Vec<Note>,
}

impl User {
    // ユーザーの新規作成
    fn new(id: &str) -> Self {
        User {
            id: id.to_string(),
            notes: Vec::new(),
        }
    }

    // ノートの追加
    fn add_note(&mut self, note: Note) {
        self.notes.push(note);
    }

    // 指定されたコミットメントに一致するノートの削除
    fn remove_note(&mut self, commit: u64) -> Option<Note> {
        if let Some(pos) = self.notes.iter().position(|x| x.commit() == commit) {
            Some(self.notes.remove(pos))
        } else {
            None
        }
    }

    // 特定の資産タイプのノートを統合し、新しいノートを作成
    fn merge_notes(&mut self, asset_type: &str) {
        let mut amount = 0;
        self.notes.retain(|note| {
            if note.asset_type == asset_type {
                amount += note.amount;
                false
            } else {
                true
            }
        });
        if amount > 0 {
            self.add_note(Note {
                asset_type: asset_type.to_string(),
                amount,
            });
        }
    }
}

// トランザクションの検証関数。正当なトランザクションであるかを検証し、対応する処理を実行
fn verify_transaction(users: &mut HashMap<String, User>, transaction: &Transaction) -> bool {
    if let Some(user) = users.get_mut(&transaction.to) {
        user.add_note(Note {
            asset_type: "BTC".to_string(), // これはトランザクションの内容から導出する必要がある
            amount: 50,                    // これもトランザクションの内容から導出する必要がある
        });
        if let Some(user) = users.get_mut("Alice") {
            user.remove_note(transaction.from_commit);
            user.add_note(Note {
                asset_type: "BTC".to_string(), // これはトランザクションの内容から導出する必要がある
                amount: 50,                    // これもトランザクションの内容から導出する必要がある
            });
            true
        } else {
            println!(
                "Transaction verification failed with user {}",
                transaction.to
            );
            false
        }
    } else {
        println!("Transaction verification failed with no user");
        false
    }
}

fn main() {
    // ユーザーとノートの初期設定
    let mut users = HashMap::new();
    let mut alice = User::new("Alice");
    let mut bob = User::new("Bob");

    alice.add_note(Note {
        asset_type: "BTC".to_string(),
        amount: 100,
    });

    let note_to_bob = Note {
        asset_type: "BTC".to_string(),
        amount: 50,
    };
    // ユーザーの情報をHashMapに追加
    users.insert("Alice".to_string(), alice);
    users.insert("Bob".to_string(), bob);

    // トランザクションの作成と実行
    let transaction = Transaction::new(&users["Alice"].notes[0], "Bob", &note_to_bob);

    // トランザクションを検証して、適切にノートを移動
    if verify_transaction(&mut users, &transaction) {
        println!("Transaction verified and completed");
    } else {
        println!("Transaction verification failed");
    }

    // トランザクション後のユーザー情報を表示
    println!("Users after transaction: {:?}", users);
}
