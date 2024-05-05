using Unity;

/// <summary>
/// TODO: Add an "unload"/"load" function to scene's managers for special behaviours during these states. 
/// </summary>
public class LoadingState : IState
{
    public LoadingState( Game gameManager )
    {
        Game = gameManager;
    }

    public string          Name            { get; set; } = "Loading State";
    public Game            Game            { get; set; }
    public StateController StateController { get; set; }

    public string ScenePath;

    public void Enter()
    {
        throw new System.NotImplementedException();
    }

    public void Exit()
    {
        throw new System.NotImplementedException();
    }

    public void Update()
    {
        throw new System.NotImplementedException();
    }
}