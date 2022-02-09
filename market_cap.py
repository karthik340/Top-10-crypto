#!/usr/bin/env python3
import requests
import json
import sys
PRICE_URL = "https://api.coingecko.com/api/v3/coins/markets"
TOP_N_COINS = 10
def adjust_rounding(data):
    if data < 1:
        return round(data, 8)
    elif data < 10:
        return round(data, 6)
    else:
        return round(data, 4)


def main():
    names=[]
    market_cap=[]
    prices=[]
    
    markets = requests.get(
        PRICE_URL,
        params={"vs_currency":"usd", "order":"market_cap_desc","per_page":"10","page":"1", "sparkline":"false"},
    ).json()
    
    for market in markets:
        names.append(market['name'].lower())
        market_cap.append(market['market_cap'])
        prices.append(market['current_price'])


    res1= ",".join(names)
    res2= ",".join([str(adjust_rounding(px)) for px in market_cap])
    res3= ",".join([str(adjust_rounding(px)) for px in prices])
    res=res1+","+res2+","+res3
    return res


if __name__ == "__main__":
    try:
        print(main())
    except Exception as e:
        print(e, file=sys.stderr)
        sys.exit(1)

