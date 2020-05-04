table! {
  blocks (id) {
    id -> Integer,
    coin_id -> Integer,
    height -> Integer,
    time -> Integer,
    userid -> Integer,
    workerid -> Integer,
    confirmations -> Integer,
    amount -> Double,
    difficulty -> Double,
    difficulty_user -> Double,
    blockhash -> Text,
    algo -> Text,
    category -> Text,
    stratum_id -> Text,
    mode -> Text,
    party_pass -> Text,
  }
}

table! {
  earnings (id) {
    id -> Integer,
    userid -> Integer,
    coinid -> Integer,
    blockid -> Integer,
    create_time -> Integer,
    amount -> Double,
    status -> Integer,
    mode -> Text,
    stratum -> Text,
  }
}

table! {
  shares (id) {
    id -> Integer,
    user_id -> Integer,
    worker_id -> Integer,
    coin_id -> Integer,
    timestamp -> Integer,
    algo -> Integer,
    difficulty -> Double,
    share_diff -> Double,
    block_reward -> Double,
    block_diff -> Double,
    mode -> Integer,
    party_pass -> Text,
    stratum_id -> Integer,
  }
}
