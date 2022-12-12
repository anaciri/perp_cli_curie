pub fn i_exchange_abi() -> String {
    return r#"[{"anonymous":false,"inputs":[{"indexed":false,"internalType":"address","name":"accountBalance","type":"address"}],"name":"AccountBalanceChanged","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"clearingHouse","type":"address"}],"name":"ClearingHouseChanged","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"baseToken","type":"address"},{"indexed":false,"internalType":"uint256","name":"markTwap","type":"uint256"},{"indexed":false,"internalType":"uint256","name":"indexTwap","type":"uint256"}],"name":"FundingUpdated","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"baseToken","type":"address"},{"indexed":false,"internalType":"uint24","name":"maxTickCrossedWithinBlock","type":"uint24"}],"name":"MaxTickCrossedWithinBlockChanged","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"previousOwner","type":"address"},{"indexed":true,"internalType":"address","name":"newOwner","type":"address"}],"name":"OwnershipTransferred","type":"event"},{"inputs":[],"name":"candidate","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"getAccountBalance","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"trader","type":"address"}],"name":"getAllPendingFundingPayment","outputs":[{"internalType":"int256","name":"pendingFundingPayment","type":"int256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"getClearingHouse","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"getClearingHouseConfig","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"getMarketRegistry","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"baseToken","type":"address"}],"name":"getMaxTickCrossedWithinBlock","outputs":[{"internalType":"uint24","name":"","type":"uint24"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"getOrderBook","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"trader","type":"address"},{"internalType":"address","name":"baseToken","type":"address"}],"name":"getPendingFundingPayment","outputs":[{"internalType":"int256","name":"","type":"int256"}],"stateMutability":"view","type":"function"},{"inputs":[{"components":[{"internalType":"address","name":"trader","type":"address"},{"internalType":"address","name":"baseToken","type":"address"},{"internalType":"int256","name":"base","type":"int256"},{"internalType":"int256","name":"quote","type":"int256"}],"internalType":"struct IExchange.RealizePnlParams","name":"params","type":"tuple"}],"name":"getPnlToBeRealized","outputs":[{"internalType":"int256","name":"","type":"int256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"baseToken","type":"address"},{"internalType":"uint32","name":"twapInterval","type":"uint32"}],"name":"getSqrtMarkTwapX96","outputs":[{"internalType":"uint160","name":"","type":"uint160"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"marketRegistryArg","type":"address"},{"internalType":"address","name":"orderBookArg","type":"address"},{"internalType":"address","name":"clearingHouseConfigArg","type":"address"}],"name":"initialize","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"baseToken","type":"address"}],"name":"isOverPriceSpread","outputs":[{"internalType":"bool","name":"","type":"bool"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"owner","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"renounceOwnership","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"accountBalanceArg","type":"address"}],"name":"setAccountBalance","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"clearingHouseArg","type":"address"}],"name":"setClearingHouse","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"baseToken","type":"address"},{"internalType":"uint24","name":"maxTickCrossedWithinBlock","type":"uint24"}],"name":"setMaxTickCrossedWithinBlock","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"newOwner","type":"address"}],"name":"setOwner","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"trader","type":"address"},{"internalType":"address","name":"baseToken","type":"address"}],"name":"settleFunding","outputs":[{"internalType":"int256","name":"fundingPayment","type":"int256"},{"components":[{"internalType":"int256","name":"twPremiumX96","type":"int256"},{"internalType":"int256","name":"twPremiumDivBySqrtPriceX96","type":"int256"}],"internalType":"struct Funding.Growth","name":"fundingGrowthGlobal","type":"tuple"}],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"components":[{"internalType":"address","name":"trader","type":"address"},{"internalType":"address","name":"baseToken","type":"address"},{"internalType":"bool","name":"isBaseToQuote","type":"bool"},{"internalType":"bool","name":"isExactInput","type":"bool"},{"internalType":"bool","name":"isClose","type":"bool"},{"internalType":"uint256","name":"amount","type":"uint256"},{"internalType":"uint160","name":"sqrtPriceLimitX96","type":"uint160"}],"internalType":"struct IExchange.SwapParams","name":"params","type":"tuple"}],"name":"swap","outputs":[{"components":[{"internalType":"uint256","name":"base","type":"uint256"},{"internalType":"uint256","name":"quote","type":"uint256"},{"internalType":"int256","name":"exchangedPositionSize","type":"int256"},{"internalType":"int256","name":"exchangedPositionNotional","type":"int256"},{"internalType":"uint256","name":"fee","type":"uint256"},{"internalType":"uint256","name":"insuranceFundFee","type":"uint256"},{"internalType":"int256","name":"pnlToBeRealized","type":"int256"},{"internalType":"uint256","name":"sqrtPriceAfterX96","type":"uint256"},{"internalType":"int24","name":"tick","type":"int24"},{"internalType":"bool","name":"isPartialClose","type":"bool"}],"internalType":"struct IExchange.SwapResponse","name":"","type":"tuple"}],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"int256","name":"amount0Delta","type":"int256"},{"internalType":"int256","name":"amount1Delta","type":"int256"},{"internalType":"bytes","name":"data","type":"bytes"}],"name":"uniswapV3SwapCallback","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"updateOwner","outputs":[],"stateMutability":"nonpayable","type":"function"}]"#.to_string();
}