use alloy_primitives::{Bytes, B256};
use alloy_rpc_types::{BlockId, BlockNumberOrTag, TransactionRequest};
use alloy_rpc_types_trace::geth::{GethDebugTracingCallOptions, GethDebugTracingOptions, GethTrace, TraceResult};
use jsonrpsee::{core::RpcResult, proc_macros::rpc};

/// Debug API
/// Taken from Reth's DebugApi trait:
/// <https://github.com/paradigmxyz/reth/blob/5d6ac4c815c562677d7ae6ad6b422b55ef4ed8e2/crates/rpc/rpc-api/src/debug.rs#L14>
#[rpc(server, namespace = "debug")]
#[async_trait]
pub trait DebugApi {
    /// Returns an RLP-encoded header.
    #[method(name = "getRawHeader")]
    async fn raw_header(&self, block_id: BlockId) -> RpcResult<Bytes>;

    /// Returns an RLP-encoded block.
    #[method(name = "getRawBlock")]
    async fn raw_block(&self, block_id: BlockId) -> RpcResult<Bytes>;

    /// Returns a EIP-2718 binary-encoded transaction.
    ///
    /// If this is a pooled EIP-4844 transaction, the blob sidecar is included.
    #[method(name = "getRawTransaction")]
    async fn raw_transaction(&self, hash: B256) -> RpcResult<Option<Bytes>>;

    /// Returns an array of EIP-2718 binary-encoded transactions for the given [BlockId].
    #[method(name = "getRawTransactions")]
    async fn raw_transactions(&self, block_id: BlockId) -> RpcResult<Vec<Bytes>>;

    /// Returns an array of EIP-2718 binary-encoded receipts.
    #[method(name = "getRawReceipts")]
    async fn raw_receipts(&self, block_id: BlockId) -> RpcResult<Vec<Bytes>>;

    /// Returns the Geth debug trace for the given block number.
    #[method(name = "traceBlockByNumber")]
    async fn trace_block_by_number(
        &self,
        block_number: BlockNumberOrTag,
        opts: Option<GethDebugTracingOptions>,
    ) -> RpcResult<Vec<TraceResult>>;

    /// Returns the Geth debug trace for the given block hash.
    #[method(name = "traceBlockByHash")]
    async fn trace_block_by_hash(
        &self,
        block_hash: B256,
        opts: Option<GethDebugTracingOptions>,
    ) -> RpcResult<Vec<TraceResult>>;

    /// Returns the Geth debug trace for the given transaction hash.
    #[method(name = "traceTransaction")]
    async fn trace_transaction(
        &self,
        transaction_hash: B256,
        opts: Option<GethDebugTracingOptions>,
    ) -> RpcResult<GethTrace>;

    /// Runs an `eth_call` within the context of a given block execution and returns the Geth debug trace.
    #[method(name = "traceCall")]
    async fn trace_call(
        &self,
        request: TransactionRequest,
        block_number: Option<BlockId>,
        opts: Option<GethDebugTracingCallOptions>,
    ) -> RpcResult<GethTrace>;
}
