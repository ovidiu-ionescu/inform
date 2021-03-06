# inform

The Unifi controller current version keeps missing heartbeats from its devices
and it takes them into _Adopting_ mode.

This program will connect via ssh to the Unifi AP devices and issue a connect
command that will inform the Unifi controller the AP device wants to be 
adopted. The controller will then adopt the device and allow them to be managed.
The effect is only temporary, it's just a matter of minutes before the 
controller will miss another heartbeat and mark the devices back to _Adopting_.

The command to request an access point to be adopted has the form:
```
mca-cli-op set-inform http://ip-of-controller:8080/inform
```
and it is retrieved from settings.toml

A sample settings.toml:

```
user="admin"
password="secret"
hosts=[ "ap1.domain.com", "ap2.domain.com"" ]
address="0.0.0.0:7878"
command="mca-cli-op set-inform http://unify-controller:8080/inform"
redirect="https://my-unifi-controller.com/manage"
```

The program will reply with a temporary redirect to send the browser to the
interface of the controller after the adoption commands have been executed.

