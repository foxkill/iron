## Iron

Iron is a tool which was written in Rust and which allows it to view the results of U.S. Treasury Auctions. It uses my library auctionresult to retrieve the necessary data. You can view different tenors which were auctioned and also select the number of auction to lookback in the past. You can choose which parameter you want to use to assess the quality of the auction. Select either Bid To Cover, Primary Dealers or Indirects to get a notion of how good the auction was executed. The UI was created with Slint + Rust, but has also bindings for C++ and NodeJS. Slint support Windows, MacOS and the Linux platform ![Slint] (https://slint.dev/releases/1.5.1/docs/slint/)

This screenshot shows the Percentage of Indirects takedown for the 10yr tenor:

![Percentage of Indirect Takedown](https://github.com/foxkill/iron/assets/7531860/82afca95-40d2-4d01-a782-8e24a45e2710)

This screen shows the "Bid To Cover Ratio" for the 10yr tenor:

![Shows: Bid To Cover Ratio](https://github.com/foxkill/iron/assets/7531860/bc1bb21f-2dd8-4954-9203-7a7950bc60f3)


