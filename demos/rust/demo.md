# Demo biscuit

## Step 1: create a biscuit

```biscuit
// a token can carry authentication info
user("clementd");
// it can also directly authorization info. It really depends on the server
right("puna", "read");
// add one by one, to show it is additive
right("nix", "read");
```

## Step 2: attenuate the biscuit

```biscuit
// add this on a token with both rights
check if dog("puna");

check if time($time), $time < 2024-05-01T00:00:00Z;
```

## Step 3: inspect snapshot

Paste the snapshot in the web inspector

## Step 4: inspect snapshot in cli

```sh
read SNAPSHOT

biscuit inspect-snapshot - <<< $SNAPSHOT

biscuit inspect-snapshot --query 'data($d) <- dog($d)' - <<< $SNAPSHOT

biscuit inspect-snapshot --query 'data($d) <- dog($d)' --json - <<< $SNAPSHOT

biscuit inspect-snapshot --query 'data($d) <- dog($d)' --json - <<< $SNAPSHOT | jq '.query.facts[0]' -r
```
