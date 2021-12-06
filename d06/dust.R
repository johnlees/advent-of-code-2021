day6_dust <- odin.dust::odin_dust("initial(day) <- 0
update(day) <- (day + 1)

update(fish[]) <- fish[i]

start_ptr = day %% 9 + 1
rebirth_ptr = (day + 7) %% 9 + 1
reborn <- fish[as.integer(start_ptr)]
update(fish[as.integer(rebirth_ptr)]) <- fish[as.integer(rebirth_ptr)] + reborn
update(total_fish) <- total_fish + reborn

## Initial states:
initial(fish[]) <- fish_ini[i]
initial(total_fish) <- sum(fish_ini)
fish_ini[] <- user()
dim(fish) <- 9
dim(fish_ini) <- 9
", workdir="dust_tmp")

generator <- day6_dust$new(pars = list(fish_ini = c(0, 116, 45, 42, 48, 49, 0, 0, 0)),
                           step = 0,
                           n_particles = 1L)
generator$set_index(2L)
as.integer(generator$run(80L))
generator$run(256L) # for this to work precisely need to change double -> uint64_t


