# Oldschool Runescape High Alchemy Arbitrage

Oldschool Runescape (OSRS) is an awesome game. One of its coolest features is the Grand Exchange (GE), which is an item marketplace for players to buy and sell items they have. The Grand Exchange works like any other exchange with an order book.

Another interesting aspect of the game is the High Alchemy (high alc) spell. This is a spell in the normal spell book that a player can cast on an item and convert it directly to gold pieces (gp). The amount of gold that is created is constant and is determined by the game engine. It costs 1 nature rune (avg cost 86 gp) and 5 fire runes (0 cost due to fire staff). 

With the GE and high alc features of the game, there is an interesting opportunity to make risk-less gp by performing price arbitrage between GE prices anf high alc prices. In other words, if the GE price of an item is lower than the high alc price, a player can buy an item on the GE for price `x` and cast high alc on the item for price `y` and make `x-y` gp per cast of the spell.

There are dedicated websites that provide this information, https://alchmate.com/ is one of the nicest UIs I've seen so far, but the data always seemed to be just out of date. I also assume I'm not the only one making use of this strategy and website, and I do not trust that the website is providing me with the most up-to-date data, so I'm embarking on a journey to make my own tool.

### Architecture

I've written this program in Rust because that's what I use day-to-day, and if I need to make this a performant program I already have the tools to do so. 

To run this program, you can use the following command:

```
$ cargo run
```

I use the [oldschool runescape wiki](https://oldschool.runescape.wiki/w/RuneScape:Real-time_Prices) as a source of the latest GE data, but I may incorporate more resources later. The wiki provides a few nice APIs to get started:

1. A `/mapping` API with all the items, their fixed alch price, and their name and id.
2. A `/latest` API wih the latest high/low price seen on the GE.

I use the `/mapping` API to build my item dataset, and poll `/latest` on an interval to provide me the latest GE price data. 

I wrote a simple scheduler that will run at 1 hertz and schedule tasks such as flushing saved data to disk, updating the latest prices in memory, and calculating what the best arbitrage deals are.

I currently have a very naive arbitrage task running every 30 seconds. This will iterate through the items db and create a max_heap where the sort key is `high_alc_price - ge_price`. I then pop the top ten best opportunities. Now that the main program is written, I plan to spend the most time enhancing this task with different strategies.

### Results

My initial experiments were successful, but I've run into a few issues right away:

1. The algorithm only takes price difference into account. This leads to rankings where the item is already very expensive and only generates a small profit per item. I'd spend 8 million gp to make 60k profit. The algorithm does not take `scale` into account
2. The algorithm does not take `high_time` into account (`high_time` is the timestamp at which the item was sold at that price). I repeatedly saw recommendations that looked great, but had no basis in reality. 
3. Bad market liquidity signals. I have to manually test out the hypothesis since botting is not allowed in runescape. As such, I'd find an opportunity but only find a few items selling at the price point. I guess this is the life of an arbitrage trader, but for this to scale I need to be able to act upon opportunities fast. 