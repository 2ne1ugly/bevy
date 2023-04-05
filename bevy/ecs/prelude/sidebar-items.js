window.SIDEBAR_ITEMS = {"derive":[["Bundle",""],["Component",""],["Resource",""],["States",""],["SystemSet","Derive macro generating an impl of the trait `SystemSet`."]],"fn":[["any_with_component","Generates a `Condition`-satisfying closure that returns `true` if there are any entities with the given component type."],["apply_state_transition","If a new state is queued in [`NextState<S>`], this system:"],["apply_system_buffers","Instructs the executor to call `apply_buffers` on the systems that have run but not applied their buffers."],["dbg","System adapter that utilizes the [`bevy_utils::tracing::debug!`] macro to print the output of a system."],["error","System adapter that utilizes the [`bevy_utils::tracing::error!`] macro to print the output of a system."],["ignore","System adapter that ignores the output of the previous system in a pipe. This is useful for fallible systems that should simply return early in case of an `Err`/`None`."],["in_state","Generates a `Condition`-satisfying closure that returns `true` if the state machine is currently in `state`."],["info","System adapter that utilizes the [`bevy_utils::tracing::info!`] macro to print system information."],["not","Generates a `Condition` that inverses the result of passed one."],["on_event","Generates a `Condition`-satisfying closure that returns `true` if there are any new events of the given type since it was last called."],["resource_added","Generates a `Condition`-satisfying closure that returns `true` if the resource of the given type has been added since the condition was last checked."],["resource_changed","Generates a `Condition`-satisfying closure that returns `true` if the resource of the given type has had its value changed since the condition was last checked."],["resource_changed_or_removed","Generates a `Condition`-satisfying closure that returns `true` if the resource of the given type has had its value changed since the condition was last checked."],["resource_equals","Generates a `Condition`-satisfying closure that returns `true` if the resource is equal to `value`."],["resource_exists","Generates a `Condition`-satisfying closure that returns `true` if the resource exists."],["resource_exists_and_changed","Generates a `Condition`-satisfying closure that returns `true` if the resource of the given type has had its value changed since the condition was last checked."],["resource_exists_and_equals","Generates a `Condition`-satisfying closure that returns `true` if the resource exists and is equal to `value`."],["resource_removed","Generates a `Condition`-satisfying closure that returns `true` if the resource of the given type has been removed since the condition was last checked."],["run_once","Generates a `Condition`-satisfying closure that returns `true` if the first time the condition is run and false every time after"],["state_changed","Generates a `Condition`-satisfying closure that returns `true` if the state machine changed state."],["state_exists","Generates a `Condition`-satisfying closure that returns `true` if the state machine exists."],["state_exists_and_equals","Generates a `Condition`-satisfying closure that returns `true` if the state machine exists and is currently in `state`."],["unwrap","System adapter that unwraps the `Ok` variant of a [`Result`]. This is useful for fallible systems that should panic in the case of an error."],["warn","System adapter that utilizes the [`bevy_utils::tracing::warn!`] macro to print the output of a system."]],"struct":[["Added","A filter on a component that only retains results added after the system last ran."],["AnyOf","The `AnyOf` query parameter fetches entities with any of the component types included in T."],["Changed","A filter on a component that only retains results added or mutably dereferenced after the system last ran."],["Commands","A [`Command`] queue to perform impactful changes to the [`World`]."],["Deferred","A [`SystemParam`] that stores a buffer which gets applied to the [`World`] during `apply_system_buffers`. This is used internally by `Commands` to defer `World` mutations."],["Entity","Lightweight identifier of an entity."],["EventReader","Reads events of type `T` in order and tracks which events have already been read."],["EventWriter","Sends events of type `T`."],["Events","An event collection that represents the events that occurred within the last two [`Events::update`] calls. Events can be written to using an [`EventWriter`] and are typically cheaply read using an [`EventReader`]."],["In","Wrapper type to mark a [`SystemParam`] as an input."],["Local","A system local [`SystemParam`]."],["Mut","Unique mutable borrow of an entity’s component"],["NextState","The next state of [`State<S>`]."],["NonSend","Shared borrow of a non-[`Send`] resource."],["NonSendMut","Unique borrow of a non-[`Send`] resource."],["OnEnter","The label of a `Schedule` that runs whenever [`State<S>`] enters this state."],["OnExit","The label of a `Schedule` that runs whenever [`State<S>`] exits this state."],["OnTransition","The label of a `Schedule` that only runs whenever [`State<S>`] exits the `from` state, AND enters the `to` state."],["Or","A filter that tests if any of the given filters apply."],["ParallelCommands","An alternative to [`Commands`] that can be used in parallel contexts, such as those in `Query::par_iter`"],["ParamSet","A collection of potentially conflicting [`SystemParam`]s allowed by disjoint access."],["Query","System parameter that provides selective access to the [`Component`] data stored in a [`World`]."],["QueryState","Provides scoped access to a [`World`] state according to a given [`WorldQuery`] and query filter."],["Ref","Shared borrow of an entity’s component with access to change detection. Similar to [`Mut`] but is immutable and so doesn’t require unique access."],["ReflectComponent","A struct used to operate on reflected [`Component`] of a type."],["ReflectResource","A struct used to operate on reflected [`Resource`] of a type."],["RemovedComponents","A [`SystemParam`] that grants access to the entities that had their `T` [`Component`] removed."],["Res","Shared borrow of a [`Resource`]."],["ResMut","Unique mutable borrow of a [`Resource`]."],["Schedule","A collection of systems, and the metadata and executor needed to run them in a certain order under certain conditions."],["Schedules","Resource that stores [`Schedule`]s mapped to [`ScheduleLabel`]s."],["State","A finite-state machine whose transitions have associated schedules ([`OnEnter(state)`] and [`OnExit(state)`])."],["With","Filter that selects entities with a component `T`."],["Without","Filter that selects entities without a component `T`."],["World","Stores and exposes operations on entities, components, resources, and their associated metadata."]],"trait":[["Bundle","The `Bundle` trait enables insertion and removal of [`Component`]s from an entity."],["Component","A data type that can be used to store data for an entity."],["Condition","A system that determines if one or more scheduled systems should run."],["DetectChanges","Types that can read change detection information. This change detection is controlled by [`DetectChangesMut`] types such as [`ResMut`]."],["DetectChangesMut","Types that implement reliable change detection."],["Event","A type that can be stored in an [`Events<E>`] resource You can conveniently access events using the [`EventReader`] and [`EventWriter`] system parameter."],["FromWorld","Creates an instance of the type this trait is implemented for using data from the supplied [World]."],["IntoPipeSystem","An extension trait providing the [`IntoPipeSystem::pipe`] method to pass input from one system into the next."],["IntoSystem","Conversion trait to turn something into a [`System`]."],["IntoSystemConfigs","Types that can convert into a [`SystemConfigs`]."],["IntoSystemSet","Types that can be converted into a [`SystemSet`]."],["IntoSystemSetConfig","Types that can be converted into a [`SystemSetConfig`]."],["IntoSystemSetConfigs","Types that can convert into a [`SystemSetConfigs`]."],["Resource","A type that can be inserted into a [`World`] as a singleton."],["States","Types that can define world-wide states in a finite-state machine."],["System","An ECS system that can be added to a `Schedule`"],["SystemParamFunction","A trait implemented for all functions that can be used as [`System`]s."],["SystemSet","Types that identify logical groups of systems."]]};