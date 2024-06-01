# Charctic

Charctic ingests CSVs and makes cool graphs out of it.

- Ingested CSV may be compressed

## Example

Send data to charctic:

```shell
$ cat <(echo "hostname my-host") <(echo "timestamp $(date +%s%3N)") \
  <( top -Ek -bn1 | head -n4 \
   | awk '
      /%Cpu/ { print "cpu_usage", 100 - $8 }
      /KiB Mem/ { print "mem_total", $4 * 1024; print "mem_free", $6 * 1024 }' \
   ) | jq -R 'split(" ") | { (.[0]): (.[1] | tonumber? // .) } * {}' | jq -s add \
   | curl -XPOST --data-binary @- http://localhost:11257/ingest
```

Get your pretty visuals:

todo

<!--
```shell
$ { echo -ne "timestamp,host,cpu_usage,mem_total,mem_free\n$(date +%s),my-host,"; \
    top -Ek -bn1 | head -n4 | awk '
      /%Cpu/ { print 100 - $8 "," }
      /KiB Mem/ { print $4 * 1024 "," $6 * 1024 }' ORS=""; \
  } | curl -XPOST --data-binary @- http://localhost:11257/ingest
```
-->

## FAQ

### What kind of data can it ingest?

JSON data that contains at least a field called `timestamp` containing a UNIX
timestamp in milliseconds.

### Why JSON instead of CSV?

JSON has types.

### Is this as powerful as Grafana?

No.

### Does this scale as well as Grafana?

No.

### Is this as powerful as InfluxDB?

No.

### Does this scale as well as InfluxDB?

No.

### Is it fast?

I doubt it.
