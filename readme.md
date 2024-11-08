# This is a *minimal* working example of the Http.get_message function breaking 

This seems to be because the message has reactions and the function decode_resp doesn't get handed a valid decoder to handle the reqwest response 

to make this example work go into the EventHandler function `message` and insert valid message id's and channel id's for two messages, the first one should have no reacts, the second one atleast 1 react. 
then run the bot and trigger the event by typing in a message to a channel the bot can see. 

you must set a valid DISCORD_TOKEN in your envrionment for this to work. 


