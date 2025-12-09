# Embedded HAL and Embassy Embedded Hal

main API to provide generic MCU implementation, all hardware related implementation must be use one of that.

# Controller 

part of the program that parsing flatbuffer request, then pass to coresponding service, then serialize response from service back to flatbuffer message. 

# Service 

is first layer that do hardware logic, service should be still generic, service will use **service provider** to get concrete service implementation. 

# Service Provider 

service provider is part of program that provide **concrete service**, it resolve service from container, then return concrete service as generic interface, so **Service** able to do operation without need to know concrete implementation 

# Container 

this is hardware specific implementation of service, **Service Provider** will look to container for resolving required service, all service inside container must be follow **Service** interface, so no mather how concrete service is implemented inside container, generic 
program will able to communicate with it.
