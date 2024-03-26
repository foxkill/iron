## Iron

Iron is a tool which was written in Rust and which allows it to view the results of U.S. Treasury Auctions. It uses my library auctionresult to retrieve the necessary data. You can view different tenors which were auctioned and also select the number of auction to lookback in the past. You can choose which parameter you want to use to assess the quality of the auction. Select either Bid To Cover, Primary Dealers or Indirects to get a notion of how good the auction was executed. The UI was created with Slint + Rust, but has also bindings for C++ and NodeJS. Slint support Windows, MacOS and the Linux platform ![Slint] (https://slint.dev/releases/1.5.1/docs/slint/)

This screenshot show the Percentage of Indirects takedown for the 10yr tenor:

![Indirects](https://github.com/foxkill/iron/assets/7531860/e4491d1d-dcd1-454d-995a-96043a07ac49)

The next screen shows the Bid To Cover Ratio for the 10yr tenor

![Bid to Cover](https://github.com/foxkill/iron/assets/7531860/616c5ad9-7ac3-432b-b3da-cb177e23b3cd)


