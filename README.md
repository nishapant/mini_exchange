# Mini Exchange 

We developed an orderbook-based mini-exchange using the programming language Rust. The program consists of four primary applications: a gateway, an order matching engine, a dropcopy, and a market data ticker plant, as well as client machines to interact with the exchange.

Traders connect directly to the gateway using TCP to send orders and receive order updates. Currently traders input orders using a basic command line interface. After the order is submitted, it is sent to the Gateway where we validate the information contained in the order. If it is valid, the trade is sent to the order matching engine using UDP multicast. The matching engine receives orders and determines if the incoming order matches with any resting orders on the book. If no trades can be executed, and the incoming order is a limit order, we rest the order on the orderbook. Otherwise we use the gateway to send information back to the client indicating that the trade was not executed. The tickerplant publishes public market data using UDP multicast whenever a trade is executed. The dropcopy aggregates all private market data related to a single company and sends it to the main trader/client from the company.




