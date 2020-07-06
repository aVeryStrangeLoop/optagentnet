
Forgettable things:
1. the Population::get_sw() function and get_sw_mt() functions are equivalent except mt allows multithreading. For large populations its generally a good idea to use mt.
2. the Grid::get_sw() function and get_sw_mt() functions are equivalent except mt allows multithreading. It is not recommended to use mt unless the resvec -> utility function is time consuming.
3. MULTITHREADING IS NOT ALWAYS FASTER. 8-10 threads work best (from what I've played with till now). 
