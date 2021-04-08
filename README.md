# Rust GoDaddy DNS Updater

Update GoDaddy DNS record with dynamic IP address. 

## Instructions

- First you must have a domain registered with GoDaddy. 
- Inside GoDaddy go to DNS Management and add an (A) record. I like to use rdp as the name but it can be anything you want.
- Go to https://developer.godaddy.com/ and generate an Key and Secret.
- Create a json file called config.json and the values below and put it in the main directory of your project.

```json
{
	"key": "{my key}",
	"secret": "{my secret }",
	"domain": "{name of domain without subdomain name -- foo.com not www.foo.com}",
	"record_type": "A",
	"subdomain": "{value -- example 'rdp'}"
}
```

## Notes
I personally have this set up as a windows scheduled task that runs every hour. If you're going to use it in a production build make sure you make a copy of the config.json and put it in the release/debug folder.
