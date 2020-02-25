# Bith.Order.Worker


## TODO

1. Base
*  [x] testing the public api request 
*  [x] error parsing only rest error
*  [x] rearrange modules
*  [x] encode the apikeys inside the new method. (use base64) : no base64..
*  [x] request for public and private
*  [x] signatures

2. PUBLIC API
*  [x] request method
*  [ ] typed request
*  [ ] ticker
*  [ ] order book
*  [ ] transaction history
*  [ ] btci

3. PRIVATE API
*  [x] request method
*  [x] typed request
INFO
*  [x] account
*  [ ] balance
*  [ ] wallet_address
*  [ ] ticker
*  [ ] orders
*  [ ] orders_detail(executed)
*  [ ] transactions
TRADE
*  [ ] place 
*  [ ] cancel
*  [ ] market buy
*  [ ] market sell
*  [ ] withdrawal(coin)
*  [ ] withdrawal(krw)

## Structures
- Dealer
- Router
- REP


## Message

```json
{
	"op": "",
	"mid": "{message id}",
	"args": {
		//...
	}
}
```

## Bin


1. Worker server
```bash
cargo run --bin worker {server_endpoint}
```
