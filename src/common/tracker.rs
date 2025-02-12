//! Track every transaction from the current block till the creation of the account.
//!
//!

use solana_client::{
    rpc_client::GetConfirmedSignaturesForAddress2Config, rpc_config::RpcTransactionConfig,
};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Signature};
use solana_transaction_status_client_types::UiTransactionEncoding;

use crate::ARpcCon;

//
pub fn wallet_history(con: ARpcCon, pub_address: &Pubkey) {
    let mut config = GetConfirmedSignaturesForAddress2Config {
        before: None,
        until: None,
        limit: Some(1000usize),
        commitment: Some(CommitmentConfig::confirmed()),
    };

    let result = con
        .get_signatures_for_address_with_config(pub_address, config)
        .unwrap();

    println!("Tx: {:#?}", result);
}

// Inspect the transaction data.
// solana confirm -v 5LAz3miLk4LMZ7992qXSTnCTZEcMRhqP1G7qX3GP6pTgJaRRxzTtFFsNTiVjPPB6NHuD1ibKFXM1BbakuPb5ezAx --url mainnet-beta
//
fn inspect_transaction(con: ARpcCon, signature: &Signature) {
    let config = RpcTransactionConfig {
        encoding: Some(UiTransactionEncoding::Json),
        commitment: Some(CommitmentConfig::confirmed()),
        max_supported_transaction_version: Some(0),
    };
    let data = con.get_transaction_with_config(signature, config).unwrap();
    println!("{:#?}", data);
}

#[cfg(test)]
mod tests {

    use super::*;
    use solana_client::rpc_client::RpcClient;
    use std::sync::Arc;

    #[test]
    #[ignore = "Not needed now"]
    fn test_wallet_history() {
        // establish rpc connection
        let con = Arc::new(RpcClient::new(
            "https://solana-mainnet.core.chainstack.com/8d701a8cf39221fedef455984ecd8b4f",
        ));
        // AbEHd4jf9yPdpmj4x9cwbxF51ZWkCm6wn4SjqRq11ZAG
        // 77dm47e18rvGuWRa2Ro1L5AVr4i79onb6bLCwVGxs5Jk
        let pub_address = "AbEHd4jf9yPdpmj4x9cwbxF51ZWkCm6wn4SjqRq11ZAG";

        wallet_history(con, &pub_address.parse::<Pubkey>().unwrap());
    }

    #[test]
    pub fn test_inspect_transaction() {
        let con = Arc::new(RpcClient::new(
            "https://solana-mainnet.core.chainstack.com/8d701a8cf39221fedef455984ecd8b4f",
        ));
        let signature = "5LAz3miLk4LMZ7992qXSTnCTZEcMRhqP1G7qX3GP6pTgJaRRxzTtFFsNTiVjPPB6NHuD1ibKFXM1BbakuPb5ezAx".parse::<Signature>().unwrap();
        inspect_transaction(con, &signature);
    }
}

/*
Tx: [
    RpcConfirmedTransactionStatusWithSignature {
        signature: "32k1gGT7nLZ5mKwNNMYU2BaLZxB5NAxN7LNwNgZPmWSNwEHQgVAGgGVApwhiaQF59c6CgeSPbmbd8V28gWfyfsPF",
        slot: 319505924,
        err: None,
        memo: None,
        block_time: Some(
            1739103417,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "4psXMgXbq8MLpxaYGYP3zV2YLSUnzscmz94bYHmQL8mhmqGiz63nzTkiExSB2ex4VXbZ21KT9q9J9WDvPvgJK4Em",
        slot: 319398220,
        err: None,
        memo: None,
        block_time: Some(
            1739060696,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "3ndr53XKSLPd53KrSPUMoEPsS9qQ4y2gsSX8pUNjJe9vZDyGHWJC4KFu4oSYtWbunXQifxQtbBp51L8raE22CTFL",
        slot: 319339122,
        err: None,
        memo: None,
        block_time: Some(
            1739037146,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "4KWPqRq3CmLcjYhN9AudEfrexRpGGhzP3akviGaCVjYDEGKZbPvLgNTZ83fGtw5obnwak3jJ3kYD6uf97cYmP93d",
        slot: 319332627,
        err: None,
        memo: None,
        block_time: Some(
            1739034569,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "57fjHFQCEQbAWcQ5VGDgGvtv5GgHhAsZRTP868DouNoyA43exsuDkeDNFGt6xxQjJM7MFKCZLVhoZ3Y1PtRsvwZ7",
        slot: 319332510,
        err: None,
        memo: None,
        block_time: Some(
            1739034522,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "9AzwQShmLcxfMqNdyduW6NpKTzezB51E23BXUhCDWcPFrgFdVVp1z42oWp44ER4ywzLxuMNyJGoT5SFjr8o3FUa",
        slot: 319332510,
        err: None,
        memo: None,
        block_time: Some(
            1739034522,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "4NVfx8jgwRJxMLhb65veagHQZPChPfqPjewJdf3isF67ao2dtx9RrZrRnEh7xcPLUpnU4pWTpTv85e1YBYRLuQto",
        slot: 319332510,
        err: None,
        memo: None,
        block_time: Some(
            1739034522,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "67TwDr7iKJxNBtQjK1B2AwcyvdvJh7c4kxLGKSuWoJSVKiCL13WzPekrmka2Xpi3iNDVHAyZiPPM3ThjRZkxUuAe",
        slot: 319332510,
        err: Some(
            InsufficientFundsForRent {
                account_index: 4,
            },
        ),
        memo: None,
        block_time: Some(
            1739034522,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "3JmWKNefgKggG4XY1rrtXyKpCNK2Fc2B9Es7EjRC6C3KwuX6xQnoK7woC3TeZxxHnD3PUhedfiA3p5rLec5CTKPC",
        slot: 319332502,
        err: None,
        memo: None,
        block_time: Some(
            1739034519,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "2xzRsLSXmFKw8ENqqeQRZMx1ktbuiyZ3Y8kGRbJx5ZrVXCatfRd4JV8XQFmXCHnkYv8mxinkLp6FQBQ5pLnYuzaS",
        slot: 319296899,
        err: None,
        memo: None,
        block_time: Some(
            1739020381,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "2asvEtxFZ84HE64VAwiSYnxW1EDTGCdpBPVb2tcGKi8s9HvtSxFhdMeaFQir8uE1T6PLFzLmFprW2h1ne5koZWCo",
        slot: 319293312,
        err: None,
        memo: None,
        block_time: Some(
            1739018959,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "NWwJyntqmk7T5Hyap657PXirHvuYLj4t8KzRNUKpq5HuyCa88okRVB37JDLHwQ1VBFWAouckUioYZ1DYR9Gj62S",
        slot: 319293312,
        err: None,
        memo: None,
        block_time: Some(
            1739018959,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "neAFtqkuunTuiqSYMHgMXavo5h8xsz6urrWbBiCFGfAisvg2i8SkA9GaiFWeWogBcmzXBdUheg3FVm2bW13zmCQ",
        slot: 319293300,
        err: None,
        memo: None,
        block_time: Some(
            1739018954,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "5oBuYUVZuAuZUtWcc21FV71M522pPtKsn7XEh9k3xzDch7JTqdtMv4mhNngiuqhwnXa5NWDbSpqg1t1tGX2dmhMR",
        slot: 319288392,
        err: None,
        memo: None,
        block_time: Some(
            1739017009,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "5rLdwmzSzLjxPTqSJ5hP3oNFCQf43EuegnBT44CW2xVRTV9kePmzRSzjTMPNLipf5nsDZhyoX12SLos9WAfhLwLd",
        slot: 319212076,
        err: Some(
            InstructionError(
                2,
                Custom(
                    3012,
                ),
            ),
        ),
        memo: None,
        block_time: Some(
            1738986842,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "3LhYQgYo6N4T91PEGw2rtevKYU7thYFzR4GJaZCj3sVBYXVdWKcf4fxS6MrMHSsNSWe4ZF8rTZBCz82rAUxKQCKF",
        slot: 319212076,
        err: None,
        memo: None,
        block_time: Some(
            1738986842,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "2NGi1JvmPdMGUTZb2xJBkzUqzQrSQrAgbUVGm1F2PeiAEMpmMzRZywP4egCNmjFSrg4aAQgfeb985i3Yv5KsQuKh",
        slot: 319212071,
        err: None,
        memo: None,
        block_time: Some(
            1738986840,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "FBvEPRPhWYcbaoZpDH225wHQBQySaMf1Ccb8f7eEGKC3G78fQuZo5HVdQP2H4teFEJybR8o9wKz8aC2Uh87sYMD",
        slot: 319210914,
        err: None,
        memo: None,
        block_time: Some(
            1738986378,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "3Fvkc3BXFxBfA9VLBbmGG8qoCUnaHkAP6jV3Cmog9dP2zV6X9ScW4dMiQFDLNFHxWJHHjYRc3HwuM4QPGG7stWSz",
        slot: 319210807,
        err: Some(
            InstructionError(
                2,
                Custom(
                    3012,
                ),
            ),
        ),
        memo: None,
        block_time: Some(
            1738986336,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "2aXepTtaPfa5VkcLGKWg5ZuRVKYZ85k6h19y5eMJehac8bN7gkVJr7Lg9K4ZpsDmCTw5tqMDA8NqRYYEBhsRrgmq",
        slot: 319210804,
        err: None,
        memo: None,
        block_time: Some(
            1738986334,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "38WFzuWgAtjMyJS2h2s6D8NFsQJgtkZ19ZNn7BvogLP2JwqeZ6F38s8GpD211hsaDxdx6HzJJPGx8oniCycadr5q",
        slot: 319210798,
        err: None,
        memo: None,
        block_time: Some(
            1738986332,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "4Vo34cryajZNrFA8b4C1BdmhspcKGHHfYxW8g2uVCgvZBoZpukgb6qg33HGPF5Sqh2vFjwZF3RdR3mtQWcZJRe4L",
        slot: 319206124,
        err: None,
        memo: None,
        block_time: Some(
            1738984476,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "4J7jevn9AdC9pz1BdsFM9KspDmboC2Ax4BomysvmZtY2Eyyuc5NEr5uM1qtZRNYfk49NT1JNajd2KedHpsBYneWD",
        slot: 319206124,
        err: None,
        memo: None,
        block_time: Some(
            1738984476,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "2j4amuvbPaVjY4A19k752N6LSfM91EsncakqJZSPboPDuK9WrfHFBjz4kkEd7ctAwWVg7WM2RSvymZg2uWmJLjLE",
        slot: 319206123,
        err: None,
        memo: None,
        block_time: Some(
            1738984475,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "3EeRzD4wSsG69iMKY5nzEPm6xqQdVWja8wPMSf49hpL6Z8LM8WmJxtGexKzxMYzP28qMRjJKDJGsnG5AGaLB8HLe",
        slot: 319206123,
        err: None,
        memo: None,
        block_time: Some(
            1738984475,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "5BQiHRwmGAiTo8zsxHUmkEVgTVohzD45TjraNkccWCJ5TBVnhkBbeQrf6aZKY6pdhi72k1G3RabF6rWEvsh15XUQ",
        slot: 319206123,
        err: None,
        memo: None,
        block_time: Some(
            1738984475,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "TYWZUGws6YpYg2ULZGzVup6b5kCkhrSoZNWspG6jJpHLMMrv9vNKGmigEhMFY1fvJMfNf1mx8jkderpJ8F5j4u6",
        slot: 319206123,
        err: None,
        memo: None,
        block_time: Some(
            1738984475,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "2By7w36h1znd94BQpWKwkGVKjXXNEDWMJSSQY3kkE6HxXdPmEZs2kFkiGB74rHRTUxig3bBV4YhqkNusvvvViYfE",
        slot: 319206123,
        err: None,
        memo: None,
        block_time: Some(
            1738984475,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "3ysRBihbA6yRiRRbUn6h6BJaFwUhoPtR62jngExPFQG9d3UF9g5933SSTev6H9F9NKq1JFbgCDm2Kj4PcW8wLjau",
        slot: 319206123,
        err: None,
        memo: None,
        block_time: Some(
            1738984475,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "3zi1xSGRh5kHFBKqevWw5CAyypfNDbAPP5jmUVS6AJpovh3nGRGJaGhbEEQZMn1kEjqZq91qZnvNgsvTHJsspuWq",
        slot: 319206117,
        err: None,
        memo: None,
        block_time: Some(
            1738984473,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "4Tf9dLH2KNovkiU8m9dAMakGE7RchPF6Jtvz4PbfB6WVzo19Fh7sN83xf3tCxFqwadKTMigNgU5e5VRDpHBEkUTM",
        slot: 319178760,
        err: None,
        memo: None,
        block_time: Some(
            1738973605,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "5nbFh7XyZcbVXHweQK75i9MNaWGQXEGB9ZGha3Zjkkc7JfT2SZDg1mX4uuyUrf4nrXJeDiNDn9jhpzSuyW3s2ioG",
        slot: 319125001,
        err: Some(
            InstructionError(
                2,
                Custom(
                    3012,
                ),
            ),
        ),
        memo: None,
        block_time: Some(
            1738952295,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "4qef2pFyrWEaJK75BGfRVDwdByvX6vdUtcEXdX4nbGaM9QYNQ56SpyqjioTTh78ahSEsVYhsjbSB76hcU2nNsLZb",
        slot: 319124999,
        err: Some(
            InstructionError(
                2,
                Custom(
                    3012,
                ),
            ),
        ),
        memo: None,
        block_time: Some(
            1738952295,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "hPrCLYasBVGUBnuMt8sxG5QR5KwLzWgzMzhFpri6Ls2ZKBhQUeSHfdxVbcxUNhibCi98fC29vyhD3W34NjEWeit",
        slot: 319124999,
        err: None,
        memo: None,
        block_time: Some(
            1738952295,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "2hQvPn1vTX5GoXa4XjueRbrSDrdH8RFhfDz2JMWQy19WJNCSqHnWQN7i6GVvozGRkLaaVm4du8S9yJ1FAe8uCBVr",
        slot: 319124995,
        err: None,
        memo: None,
        block_time: Some(
            1738952293,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "qqTL4nN2hYDPC3SswATNrUrCuutsRfwwx1B7UdHjsSbQgQRLLBFb8CCXBp5EPhTeYQiGphnne9tTvzhdYfrPntm",
        slot: 319120216,
        err: None,
        memo: None,
        block_time: Some(
            1738950389,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "dt8wSagYhb2qjWaeRpXjFYfVB1ukAxkpXuZpfxxqfYW3B9jf3VbpFehGDXiUs5AunRGZz7kDuwjuM81NzGffdt2",
        slot: 319120216,
        err: None,
        memo: None,
        block_time: Some(
            1738950389,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "4YQaBVTfCUKVkEmk6mL1PPGGPuWYzSiDqUZ5CdRYWr28SBU2VYhuSAt19q5vNzdnud3zCGgzzCLGUBw6t12Qrtxi",
        slot: 319120208,
        err: None,
        memo: None,
        block_time: Some(
            1738950386,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "3hWzPqxjrabNmFJxPg1CBxxWr8SBTJiC5CRABvB6kgxCnvZiRtcEvfEdEb2GZmNiMRQFGHSEucPTZkPansjtHVvh",
        slot: 319114947,
        err: Some(
            InstructionError(
                2,
                Custom(
                    3012,
                ),
            ),
        ),
        memo: None,
        block_time: Some(
            1738948303,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "49JekAyvuWXQHPxc1q9eeydg33mYTM8TXX1qFmUSKSsaZGqAK52wxTYF8SzU8TbWwLcMA1XF3Vj5JVQQcNrAqdj3",
        slot: 319114947,
        err: Some(
            InstructionError(
                2,
                Custom(
                    3012,
                ),
            ),
        ),
        memo: None,
        block_time: Some(
            1738948303,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "5uo9gKN5bCCjExci32vDkAxAQyjoQhaJtVR7Pn4iLu91RJkWSvxB5GVDuxhTXXfrFyP9W8PUwQBMRoWf4UaRHNAn",
        slot: 319114946,
        err: None,
        memo: None,
        block_time: Some(
            1738948303,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "5kTdxia1qFNPPCeQd46THFqFfqUS369hCkjQxJsFrqNBymVor61SQ3JUj6hJQv6qPYsCwN5qJPjit6emgwbmu4se",
        slot: 319114944,
        err: None,
        memo: None,
        block_time: Some(
            1738948302,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "2Ze64fsicPsWUGnYx3xvpqzVAmxXP2JJo2Zr4uhezzhr3PDTYChjNgD2PqAisvPWfrDq7EYpTL6wcWeMTysZSe8R",
        slot: 319108256,
        err: None,
        memo: None,
        block_time: Some(
            1738945645,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "2BNKBKuPbUoewED8C7n29d9QLSBdLyGLBM2KLZrLNStgXqGpz5joyvPt4xJGQsxvPvvyGz6oNuLxqCaCpQV7Fikz",
        slot: 319108256,
        err: None,
        memo: None,
        block_time: Some(
            1738945645,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "41ms6g5V9ZYq82Yx1epEwGruB4dRBAUtLNkdet894YQupKbrKsueaxi4awxdRMcqzjSjbw2zJDvvwtFn1fWNEUrm",
        slot: 319108248,
        err: None,
        memo: None,
        block_time: Some(
            1738945642,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "4bsX33t2Zmbi2ezQfUnQVZqNyHyC3zrYz7ys9gBDzNqgdAhNLVwqcQsBzATtXxJ7hjvy1mWj6zcxs7gDZYtZKgma",
        slot: 319106917,
        err: Some(
            InstructionError(
                2,
                Custom(
                    3012,
                ),
            ),
        ),
        memo: None,
        block_time: Some(
            1738945114,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "596uL68J8wHcJaKaE9K9cy9eGrHEe5WVbgxDNCXo1PpaUQJa9DXDh3moWRUQ3HfrvUTPP3jSTo3sXmD2qJoAg8Qv",
        slot: 319106917,
        err: Some(
            InstructionError(
                2,
                Custom(
                    3012,
                ),
            ),
        ),
        memo: None,
        block_time: Some(
            1738945114,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "3BhuPtDVftRUbmGsmbzkHZWSckGqyTxFyTTL4UfsRy9Aj3oQP8QstoTDUUcWi6eBdWPMNY23inWRGy6NneUemxeU",
        slot: 319106917,
        err: Some(
            InstructionError(
                2,
                Custom(
                    3012,
                ),
            ),
        ),
        memo: None,
        block_time: Some(
            1738945114,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "3AFPdfhaVPTvDTui7ph9Wo2tgt84U5cEzV2676FHY2vJyWqZGfPJcGaX4M83nkzkFp8RcSvPKf5doH7TVi99HBnk",
        slot: 319106916,
        err: Some(
            InstructionError(
                2,
                Custom(
                    3012,
                ),
            ),
        ),
        memo: None,
        block_time: Some(
            1738945114,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "3pwf4xKReta6xEtYVVnNGjddWMgWjLwkP4e3gjECKYVHp8xnUsDy8QTHV8yd5awKMnmHzz3pLSN8DwT8oUF8iSrQ",
        slot: 319106912,
        err: None,
        memo: None,
        block_time: Some(
            1738945112,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "4w7h6ovbQrLCjuW9TyFd8Kztg29GVkyD2UZU1NFbuBjYbtiCEoFyb7HJfWDsKRkHbXeodyhe4HhCxAgqbWXRVrhq",
        slot: 319106908,
        err: None,
        memo: None,
        block_time: Some(
            1738945110,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "2rHvx5rdCPWuteXPTgSUAu4iwbTVzDcwJJsSR3DDzVdKbrLnXiSmAf5vecyDtUQjDfFQGJHJFWVXBGdmvXVH8PQe",
        slot: 319096490,
        err: Some(
            InstructionError(
                2,
                Custom(
                    3012,
                ),
            ),
        ),
        memo: None,
        block_time: Some(
            1738940981,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "42hbjXeHHDpsHxvyTqvKFiwPdQh9dQ3ffPL37WePJGbA7fK146SPUeqVJnT3iiV8YXXEgrHcf6DZxmQGmLp3Bv3",
        slot: 319096489,
        err: None,
        memo: None,
        block_time: Some(
            1738940981,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "2wVd1mCjvf7w2zJQmHkSNVbHWFg9sCBsPuPWjA9uoxP39PxrmVvwfnGfffz58MH35YGytLeKP4Q2qTrnGVwmvkpD",
        slot: 319096486,
        err: None,
        memo: None,
        block_time: Some(
            1738940980,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "r74y2UtHVCxiGH4vCGT9V5kaoLTfykFVpTwe4p8XqVrtjz1KHH6YvnQTtC2x86GjD6X8sdycXcngigcrDyGZVWN",
        slot: 319095777,
        err: None,
        memo: None,
        block_time: Some(
            1738940699,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "5gxX1y6beewCfs1xomGHBqH5xfGWYwBgsuv3gQou8ZX7ziCYZ3T3SHrC23uTyJqeXbeph5KnN1en2phKHrhkLsTu",
        slot: 319095777,
        err: None,
        memo: None,
        block_time: Some(
            1738940699,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
    RpcConfirmedTransactionStatusWithSignature {
        signature: "5LAz3miLk4LMZ7992qXSTnCTZEcMRhqP1G7qX3GP6pTgJaRRxzTtFFsNTiVjPPB6NHuD1ibKFXM1BbakuPb5ezAx",
        slot: 319095754,
        err: None,
        memo: None,
        block_time: Some(
            1738940690,
        ),
        confirmation_status: Some(
            Finalized,
        ),
    },
]
*/
