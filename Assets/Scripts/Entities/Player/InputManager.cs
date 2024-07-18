using System;
using System.Collections;
using System.Collections.Generic;
using ObjectExtensions;
using UnityEngine;
using UnityEngine.InputSystem;
using UnityEngine.Serialization;

public class InputManager : MonoBehaviour
{
    public PlayerInput PlayerInput;

    [Header( "Broadcrast Events" )]
    [SerializeField] private VoidEventChannelSO m_OpenStatScreen;

    [SerializeField] private ChangeStateEventChannelSO    m_ChangeState;
    [SerializeField] private InitiateBattleEventChannelSO m_InitiateBattle;

    [Header( "Listen to Events" )]
    [SerializeField] private VoidEventChannelSO m_StartingBattle;

    [SerializeField] private VoidEventChannelSO m_ExitingBattle;

    public void Start()
    {
        m_StartingBattle.OnEventRaised += OnStartBattle;
        m_ExitingBattle.OnEventRaised  += OnExitBattle;
    }

    private void OnStartBattle()
    {
        PlayerInput.IsValid()?.SwitchCurrentActionMap( "Battle" );
    }

    private void OnExitBattle()
    {
        PlayerInput.IsValid()?.SwitchCurrentActionMap( "InGame" );
    }

    public void OnOpenStatScreen()
    {
        m_OpenStatScreen.IsValid()?.RaiseEvent();
    }

    public void OnDebugExit()
    {
        m_ChangeState.IsValid()?.RaiseEvent( null, TransitionType.Remove );
    }

    public void OnDebugStartBattle()
    {
        var slime_prefab = Resources.Load< GameObject >( "Entities/Enemies/Slime/Slime_Prefab" );

        m_InitiateBattle.IsValid()?.RaiseEvent( new GameObject[] { gameObject }, new GameObject[] { slime_prefab } );
    }
}