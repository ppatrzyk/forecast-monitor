# forecast-monitor

### Configuration

```
cp _config.yaml config.yaml
```

and insert required values. [weatherapi.com](https://www.weatherapi.com/) and [tomorrow.io](https://www.tomorrow.io/weather-api/) require registration to obtain API keys.

```
curl -X "POST" "http://localhost:8088/ksql" \
     -H "Accept: application/vnd.ksql.v1+json" \
     -d @create_table.json
```
