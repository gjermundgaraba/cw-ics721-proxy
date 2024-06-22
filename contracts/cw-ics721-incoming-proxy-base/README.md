# CW721 Incoming Proxy

This incoming proxy allows whitelisting channels. It validates incoming `IbcPacket` and checks from which channel packet is coming from.
In case channel is not whitelisted an `UnauthorizedChannel` is thrown.

It also validates the origin of proxy request which should be set to the address of the ICS721 contract.

In addition, it has an optional list of allowed class ids. If the list is empty any class id is allowed.
If the list is not empty only class ids from the list are allowed.