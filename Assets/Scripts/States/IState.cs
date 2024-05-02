using System;
using UnityEngine;
using UnityEngine.Events;

public interface IState
{
    public string Name { get; set; }
    
    public void   Enter();

    public void Exit();

    public void Update();
}