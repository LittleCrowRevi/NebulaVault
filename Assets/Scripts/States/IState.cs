using System;
using UnityEngine;
using UnityEngine.Events;

public interface IState
{
    public string Name { get; set; }

    public Game            Game            { get; set; }
    public StateController StateController { get; set; }

    public void Enter();

    public void Exit();

    public void Update();
}