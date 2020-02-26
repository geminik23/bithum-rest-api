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
*  [x] orders
*  [ ] orders_detail(executed)
*  [ ] transactions
TRADE
*  [x] place 
*  [x] cancel
*  [ ] market buy
*  [ ] market sell
*  [ ] withdrawal(coin)
*  [ ] withdrawal(krw)

4. Websocket API
*  [x] dependencies
*  [x] check, after connecting, ws still alive?
*  [x] listener trait and base structure
*  [ ] InnerHandler
*  [ ] ClientRT connections
*  [ ] ClientRT internal state: subscription.. trait
*  [ ] ClientRT internal methods...
*  [ ] innerRTs..

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
